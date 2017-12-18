#![allow(dead_code)]
#![allow(unknown_lints)]
#![allow(unused_imports)]
#![allow(needless_range_loop)]

#[macro_use]
extern crate failure;
use failure::Error;

use std::{io, process};
use std::io::BufRead;
//use std::slice::*;


fn read_input() -> Result<Vec<Vec<i32>>, Error> {
    let stdin = io::stdin();
    let mut input = Vec::new();
    for line in stdin.lock().lines() {
        let line = line?;
        let mut row = Vec::new();
        for word in line.split_whitespace() {
            row.push(word.parse()?);
        }
        input.push(row);
    }

    let mut result = Vec::new();
    for group in input.chunks(3) {
        let mut col0 = vec![group[0][0], group[1][0], group[2][0]];
        let mut col1 = vec![group[0][1], group[1][1], group[2][1]];
        let mut col2 = vec![group[0][2], group[1][2], group[2][2]];
        col0.sort();
        col1.sort();
        col2.sort();
        result.push(col0);
        result.push(col1);
        result.push(col2);
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
