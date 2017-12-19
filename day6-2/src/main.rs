#![allow(dead_code)]
#![allow(unknown_lints)]
#![allow(unused_imports)]
#![allow(needless_range_loop)]

#[macro_use]
extern crate failure;
use failure::Error;

use std::{io, process};
use std::io::BufRead;
use std::collections::HashMap;


fn read_input() -> Result<(usize, Vec<String>), Error> {
    let stdin = io::stdin();
    let mut lines = Vec::new();
    let mut length = 0;
    for line in stdin.lock().lines() {
        let line = line?;
        let line = line.trim();
        if !line.is_empty() {
            length = line.len();
            lines.push(line.to_string());
        }
    }

    Ok((length, lines))
}

fn decode(lines: &[String], letter: usize) -> char {
    let mut occurrences = HashMap::new();
    for ch in "abcdefghijklmnopqrstuvwxyz".chars() {
        occurrences.insert(ch, 0);
    }

    for line in lines {
        let ch = line.chars().nth(letter).unwrap();
        *occurrences.get_mut(&ch).unwrap() += 1;

    }

    *occurrences
        .iter()
        .filter(|&(_, &count)| count > 0)
        .min_by_key(|&(_, count)| count)
        .map(|(ch, _)| ch)
        .unwrap()
}

fn run() -> Result<(), Error> {
    let (length, lines) = read_input()?;

    for letter in 0..length {
        print!("{}", decode(&lines, letter));
    }
    println!();

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
