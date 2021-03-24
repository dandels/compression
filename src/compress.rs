use crate::list::List;
use std::path::PathBuf;

pub enum Algorithm {
    Huffman,
    LZ,
}

pub fn compress(files: List<&str>, algo: Algorithm) {
    for f in files.iter() {
        let path = PathBuf::from(f);
        if !path.exists() {
            panic!("No such file or directory: {}", f);
        }
        if path.is_file() {
        } else if path.is_dir() {
        } else {
            println!("Ignoring symlink: {}", f);
        }
    }
}
