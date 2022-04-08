#![deny(clippy::all)]

mod lib;

use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader};
use std::path::Path;

use crate::lib::ComputedMask;

fn parse_file<P>(path: P) -> io::Result<Vec<ComputedMask>>
where
    P: AsRef<Path>,
{
    let file = File::open(path)?;
    let file_reader = BufReader::new(file);


    todo!()
}

fn main() {
    todo!()
}

#[cfg(test)]
mod main_tests {}
