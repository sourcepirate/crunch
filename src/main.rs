//! This simple tools is used to strip the common sequnce between files
//! in the given directory
//!
//! Uses LCS algorithm to remove sequence strings from the given
//! text file.
//!
//!
extern crate docopt;
extern crate kmpsearch;
extern crate walkdir;

use docopt::Docopt;
use serde::Deserialize;

mod common;
mod lcs;
mod replace;

const USAGE: &'static str = "
Usage: crunch <source>
";

#[derive(Debug, Deserialize)]
struct Args {
    arg_source: String,
}

fn main() {
    let args: Args = Docopt::new(USAGE)
        .and_then(|d| d.options_first(true).deserialize())
        .unwrap_or_else(|e| e.exit());
    let files: Vec<String> = common::get_text_files(args.arg_source);
    let common_string = common::get_substring_from_files(files);
    println!("{}", common_string);
}
