#![deny(clippy::all)]

mod lib;

use clap::Parser;
use std::time::Instant;

use crate::lib::parse_file;

/// Parse provided file and print a list of masks up to provided space limit.
#[derive(Parser)]
#[clap(author, version, about, long_about = None, arg_required_else_help = true)]
struct Cli {
    /// wordlist to parse
    wordlist: String,

    #[clap(short = 'l', default_value_t = u64::MAX)]
    space_limit: u64,
}

fn main() {
    let cli = Cli::parse();
    let start_time = Instant::now();
    let (sorted_masks, used_space) = parse_file(cli.wordlist, cli.space_limit).unwrap();
    let duration = start_time.elapsed();

    println!(
        "Got {} masks over {} space into {:?}.",
        sorted_masks.len(),
        used_space,
        duration
    );
}
