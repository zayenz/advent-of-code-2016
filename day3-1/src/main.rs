#![allow(dead_code)]
#![allow(unknown_lints)]
#![allow(unused_imports)]
#![allow(needless_range_loop)]

#[macro_use]
extern crate failure;
use failure::Error;

use std::{io, process};
use std::io::BufRead;


fn read_input() -> Result<Vec<Vec<i32>>, Error> {
    let stdin = io::stdin();
    let mut result = Vec::new();
    for line in stdin.lock().lines() {
        let line = line?;
        let mut candidate = Vec::new();
        for word in line.split_whitespace() {
            candidate.push(word.parse()?);
        }
        candidate.sort();
        result.push(candidate);
    }
    Ok(result)
}

fn is_triangle(candidate: &[i32]) -> bool {
    assert_eq!(candidate.len(), 3);
    candidate[0] + candidate[1] > candidate[2]
}

fn run() -> Result<(), Error> {
    let candidates = read_input()?;

    let valid_count = candidates.iter().filter(|c| is_triangle(c)).count();

    println!("{}", valid_count);

    Ok(())
}


fn main() {
    match run() {
        Ok(()) => process::exit(0),
        Err(error) => {
            for cause in error.causes() {
                eprintln!("{}", cause)
            }
            process::exit(1)
        }
    }
}
