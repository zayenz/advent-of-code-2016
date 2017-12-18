#![allow(dead_code)]
#![allow(unknown_lints)]
#![allow(unused_imports)]
#![allow(needless_range_loop)]

#[macro_use]
extern crate failure;
use failure::Error;

extern crate crypto;

use crypto::md5::Md5;
use crypto::digest::Digest;

use std::{io, process, char};
use std::io::BufRead;
use std::collections::HashMap;
use std::str::FromStr;


fn read_input() -> Result<String, Error> {
    let stdin = io::stdin();
    let mut result = String::new();
    stdin.read_line(&mut result)?;
    Ok(result.trim().to_string())
}

fn run() -> Result<(), Error> {
    let door = read_input()?;

    const SENTINEL: char = ' ';
    let mut characters = [SENTINEL; 8];
    let mut found_values = 0;
    let mut index = 0;
    while found_values < 8 {
        let candidate = format!("{}{}", door, index);
        let mut hasher = Md5::new();
        hasher.input(candidate.as_bytes());
        let mut output = [0; 16]; // An MD5 is 16 bytes
        hasher.result(&mut output);
        let valid_candidate = output[0] == 0 && output[1] == 0 && (output[2] & 0xF0) == 0;
        if valid_candidate {
            let pos = (output[2] & 0x0F) as usize;
            if pos < 8 && characters[pos] == SENTINEL {
                let value = (output[3] >> 4) as u32;
                let ch = char::from_digit(value, 16).unwrap();
                characters[pos] = ch;
                found_values += 1;
            }
        }
        index += 1;
    }

    let password: String = characters.iter().collect();
    println!("{}", password);

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
