use std::alloc::{alloc, dealloc, realloc, Allocator, Layout};
use std::mem;
use std::ops::{Deref, DerefMut};
use std::ptr::{self, Unique};
use std::slice;

pub struct List<T> {
    pointer: Unique<T>,
    len: usize,
    capacity: usize,
}

impl<T> List<T> {
    pub fn new() -> Self {
        assert!(mem::size_of::<T>() != 0, "Zero sized types are unsupported.");

        Self {
            pointer: Unique::dangling(),
            len: 0,
            capacity: 0,
        }
    }

    fn grow(&mut self) {
        unsafe {
            let align = mem::align_of::<T>();
            let type_size = mem::size_of::<T>();
            let memory_layout = Layout::from_size_align(type_size, align).unwrap();

            let (new_capacity, new_pointer);
            if self.capacity == 0 {
                new_capacity = 1;
                new_pointer = alloc(memory_layout);
            } else {
                new_capacity = self.capacity * 2;
                let old_size = self.capacity * type_size;

                assert!(
                    old_size <= (isize::MAX as usize) / 2,
                    "Tried to allocate more memory than system has."
                );

                let new_size = old_size * 2;
                // TODO does the memory layout need to repeat?
                new_pointer = realloc(self.pointer.as_ptr() as *mut _, memory_layout, new_size);
            }

            if new_pointer.is_null() {
                panic!("Unable to allocate enough memory, aborting.");
            };

            self.pointer = Unique::new(new_pointer as *mut _).unwrap();
            self.capacity = new_capacity;
        }
    }

    pub fn push(&mut self, value: T) {
        if self.len == self.capacity {
            self.grow();
        };

        unsafe {
            //let mut_ptr: *mut T = self.pointer.as_mut();
            let end = self.pointer.as_ptr().offset(self.len as isize);
            ptr::write(end, value);
        }

        self.len += 1;
    }

    pub fn pop(&mut self) -> Option<T> {
        if self.len == 0 {
            None
        } else {
            self.len -= 1;
            unsafe {
                let end = (self.pointer.as_ptr() as *const T).add(self.len);
                Some(ptr::read(end))
            }
        }
    }
}

impl<T> Drop for List<T> {
    fn drop(&mut self) {
        if self.capacity != 0 {
            /* Pops values at the end of self until it returns None. Popping a value means taking ownership of it.
             * In Rust's memory model having ownership of it means Rust will free the resource after we're done using
             * it. Thus, this seemingly no-op loop actually frees the contents of our List.
             */
            while let Some(_) = self.pop() {}

            let align = mem::align_of::<T>();
            let type_size = mem::size_of::<T>();
            let byte_amount = type_size * self.capacity;
            // TODO should this use repeat() or not?
            let (memory_layout, _) = Layout::from_size_align(type_size, align)
                .unwrap()
                .repeat(byte_amount)
                .unwrap();

            unsafe {
                dealloc(self.pointer.as_ptr() as *mut _, memory_layout);
            }
        }
    }
}

/* Implementing Deref and DerefMut allows us to treat our List as a slice, which is a continuous block of memory. If
 * this confuses you, think of it as an array.
 * This allows us to treat our List as a slice, giving us access to indexing, iteration, mutable iteration and any other
 * methods which are valid for a slice.
 */
impl<T> Deref for List<T> {
    type Target = [T];

    fn deref(&self) -> &[T] {
        unsafe { slice::from_raw_parts(self.pointer.as_ptr(), self.len) }
    }
}

impl<T> DerefMut for List<T> {
    fn deref_mut(&mut self) -> &mut [T] {
        unsafe { slice::from_raw_parts_mut(self.pointer.as_ptr(), self.len) }
    }
}

struct IntoIter<T> {
    buffer: Unique<T>,
    capacity: usize,
    start: *const T,
    end: *const T,
}

/* Implementing IntoIter allows us to use our list like:
 * for foo in list {}
 */
impl<T> List<T> {
    fn into_iter(self) -> IntoIter<T> {
        /* Assigning List's fields to these variables moves ownership of self to the variables. This would cause drop()
         * to be called here, but we're not actually ready to free the memory of the List until iteration is finished.
         * By assigning the fields, then calling mem::forget() we prevent drop() from running.
         */
        let pointer = self.pointer;
        let capacity = self.capacity;
        let len = self.len;

        mem::forget(self);

        unsafe {
            let end;
            if capacity == 0 {
                end = pointer.as_ptr() as *const T;
            } else {
                end = pointer.as_ptr().offset(len as isize);
            }

            IntoIter {
                buffer: pointer,
                capacity,
                start: pointer.as_ptr(),
                end,
            }
        }
    }
}

impl<T> Iterator for IntoIter<T> {
    type Item = T;

    fn next(&mut self) -> Option<T> {
        if self.start == self.end {
            None
        } else {
            unsafe {
                let ret = ptr::read(self.start);
                self.start = self.start.offset(1);
                Some(ret)
            }
        }
    }
}

impl<T> DoubleEndedIterator for IntoIter<T> {
    fn next_back(&mut self) -> Option<T> {
        if self.start == self.end {
            None
        } else {
            unsafe {
                self.end = self.end.offset(-1);
                Some(ptr::read(self.end))
            }
        }
    }
}

impl<T> Drop for IntoIter<T> {
    fn drop(&mut self) {
        if self.capacity != 0 {
            // drop any remaining elements
            for _ in &mut *self {}

            let align = mem::align_of::<T>();
            let type_size = mem::size_of::<T>();

            let byte_amount = type_size * self.capacity;
            let (memory_layout, _) = Layout::from_size_align(type_size, align)
                .unwrap()
                .repeat(byte_amount)
                .unwrap();

            unsafe {
                dealloc(self.buffer.as_ptr() as *mut _, memory_layout);
            }
        }
    }
}

/* Since we're using unsafe code there's some possibilities for undefined behavior or memory leaks even if the unit
 * tests pass. To help catch memory leaks there's a cargo module to run Valgrind on the project.
 * https://lib.rs/crates/cargo-valgrind
 *
 * Valgrind is a powerful tool commonly used with memory unsafe languages. One of its features is finding memory leaks:
 * https://valgrind.org/
 */
#[cfg(test)]
mod test {
    use crate::list::List;

    #[test]
    fn list() {
        let mut list = List::new();
        list.push("First");
        list.push("Second");
        list.push("Third");
        list.push("Fourth");

        assert_eq!("First", list[0]);
        assert_eq!("Second", list[1]);
        assert_eq!("Third", list[2]);
        assert_eq!("Fourth", list[3]);

        assert_eq!("Fourth", list.pop().unwrap());
        assert_eq!("Third", list.pop().unwrap());
        assert_eq!("Second", list.pop().unwrap());
        assert_eq!("First", list.pop().unwrap());
    }

    // Used with `cargo valgrind test`, this checks whether "First" is deallocated when the list is dropped.
    #[test]
    fn list_drop() {
        let mut list = List::new();
        list.push("First");
    }

    #[test]
    fn list_iteration() {
        let mut list = List::new();
        list.push(1);
        list.push(2);
        list.push(3);
        list.push(4);

        let mut sum: u32 = 0;
        for f in list.into_iter() {
            sum += f;
        }
        assert_eq!(sum, 10);
    }
}
