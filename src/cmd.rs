use clap::{App, Arg, ArgGroup, ArgMatches};

pub const ARG_COMPRESS: &str = "compress";
pub const ARG_EXTRACT: &str = "extract";
pub const ARG_HUFFMAN: &str = "huffman";
pub const ARG_LZ: &str = "lempel-ziv"; // TODO decide which variant of LZ
pub const ARG_OUTPUT: &str = "output-ziv";

pub const GROUP_ALGORITHM: &str = "algorithm";
pub const GROUP_MODE: &str = "mode";

pub const VAL_ARCHIVE: &str = "archive";
pub const VAL_COMPRESS: &str = "files";
pub const VAL_OUTPUT: &str = "path";

pub fn args() -> ArgMatches<'static> {
    /* Clap is the standard Rust library for parsing command line arguments. It automatically generates helptext from
     * the command line arguments.
     * Library documentation: https://docs.rs/clap/2.33.3/clap/index.html
     *
     * Takes a mode of operation: compression or extraction.
     */
    App::new(clap::crate_name!())
        .version(clap::crate_version!())
        .about("An implementation of Huffman coding and Lempel-Ziv compression.")
        .arg(
            Arg::with_name(ARG_EXTRACT)
                .takes_value(true)
                .value_name(VAL_ARCHIVE)
                .help("Extract archive")
                .short("x"),
        )
        .arg(
            Arg::with_name(ARG_COMPRESS)
                .takes_value(true)
                .value_name(VAL_COMPRESS)
                .multiple(true)
                .help("Files & directories to compress")
                .short("c")
                .requires(GROUP_ALGORITHM)
                .requires(ARG_OUTPUT),
        )
        .arg(
            Arg::with_name(ARG_HUFFMAN)
                .short("H")
                .help("Use Huffman coding for compression"),
        )
        .arg(
            Arg::with_name(ARG_LZ)
                .short("L")
                .help("Use Lempel-Ziv for compression"),
        )
        .arg(
            Arg::with_name(ARG_OUTPUT)
                .short("o")
                .help("Archive to create.")
                .takes_value(true)
                .value_name(VAL_OUTPUT),
        )
        .group(
            ArgGroup::with_name(GROUP_MODE)
                .args(&[ARG_EXTRACT, ARG_COMPRESS])
                .required(true),
        )
        .group(ArgGroup::with_name(GROUP_ALGORITHM).args(&[ARG_HUFFMAN, ARG_LZ]))
        .get_matches()
}
