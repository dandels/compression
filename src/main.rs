#![feature(ptr_internals)]
#![feature(allocator_api)]
#![feature(alloc_layout_extra)]

mod cmd;
mod compress;
mod list;

use compress::Algorithm;

fn main() {
    let args = cmd::args();

    if let Some(files) = args.values_of(cmd::ARG_COMPRESS) {
        let algo: Algorithm;
        if args.is_present(cmd::ARG_HUFFMAN) {
            algo = Algorithm::Huffman;
        } else if args.is_present(cmd::ARG_LZ) {
            algo = Algorithm::LZ;
        } else {
            panic!("Unexpected command line arguments, clap should enforce exactly one compression algorithm is set.");
        }

        // Manually reconstruct the arg list in an ugly way because our List type doesn't support all features it should
        let mut list = list::List::new();

        for f in files {
            list.push(f);
        }

        compress::compress(list, algo);
    }
}
