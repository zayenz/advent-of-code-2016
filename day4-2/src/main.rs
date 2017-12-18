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
use std::str::FromStr;

#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
struct Room {
    name: String,
    id: usize,
    checksum: String,
}

impl Room {
    fn is_valid(&self) -> bool {
        let mut chars: Vec<char> = Vec::with_capacity(self.name.len());
        let mut occurrences: HashMap<char, isize> = HashMap::new();
        for ch in self.name.chars() {
            if ch != '-' {
                if let Some(&previous) = occurrences.get(&ch) {
                    occurrences.insert(ch, previous + 1);
                } else {
                    occurrences.insert(ch, 1);
                    chars.push(ch);
                }
            }
        }

        chars.sort_unstable_by_key(|ch| (-occurrences[ch], *ch));
        let computed_checksum: String = chars[0..5].into_iter().collect();
        computed_checksum == self.checksum
    }

    fn rotate(&self, ch: char) -> char {
        if ch == '-' {
            return ' ';
        }
        let e = (ch as u8 - b'a') as usize;
        let d = ((e + self.id) % 26) as u8;
        (d + b'a') as char
    }

    fn decrypt(&self) -> String {
        self.name.chars().map(|ch| self.rotate(ch)).collect()
    }
}

impl FromStr for Room {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.rsplitn(2, '-').collect();
        let name = parts[1].to_string();
        let id_checksum: Vec<&str> = parts[0].split(|c| c == '[' || c == ']').collect();
        let id = id_checksum[0].parse()?;
        let checksum = id_checksum[1].to_string();

        Ok(Room { name, id, checksum })
    }
}

fn read_input() -> Result<Vec<Room>, Error> {
    let stdin = io::stdin();
    let mut result = Vec::new();
    for line in stdin.lock().lines() {
        let line = line?;
        result.push(line.trim().parse()?);
    }
    Ok(result)
}

fn run() -> Result<(), Error> {
    let rooms = read_input()?;

    if let Some(room) = rooms.iter().find(
        |r| r.decrypt() == "northpole object storage",
    )
    {
        println!("{}", room.id);
        Ok(())
    } else {
        bail!("No northpole storage found")
    }
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
