#![deny(clippy::all)]

mod lib;

use std::fs::File;
use std::io;
use std::path::Path;

use crate::lib::ComputedMask;

fn parse_file<P>(path: P) -> io::Result<Vec<ComputedMask>>
where
    P: AsRef<Path>,
{
    let file = File::open(path)?;
    todo!()
}

fn main() {
    todo!()
}

#[cfg(test)]
mod main_tests {}
