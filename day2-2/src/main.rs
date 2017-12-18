#![allow(dead_code)]
#![allow(unknown_lints)]
#![allow(unused_imports)]
#![allow(needless_range_loop)]

#[macro_use]
extern crate failure;
use failure::Error;

use std::{io, process};
use std::io::BufRead;

extern crate aoc2017;
use aoc2017::UnionFind;

extern crate strum;
#[macro_use]
extern crate strum_macros;

#[derive(EnumString, Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
enum Direction {
    #[strum(serialize = "U")]
    Up,
    #[strum(serialize = "D")]
    Down,
    #[strum(serialize = "R")]
    Right,
    #[strum(serialize = "L")]
    Left,
}

use Direction::*;


#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
struct Position {
    x: i8,
    y: i8,
}

impl Position {
    fn new() -> Position {
        Position { x: 2, y: 0 }
    }

    fn position(x: i8, y: i8) -> Result<Position, Error> {
        if 0 <= x && x <= 2 && 0 <= y && y <= 2 {
            Ok(Position { x, y })
        } else {
            bail!("({}, {}) is not a valid position", x, y)
        }
    }

    fn value(&self) -> Result<char, Error> {
        Ok(match (self.x, self.y) {
            (0, 2) => '1',
            (1, 1) => '2',
            (1, 2) => '3',
            (1, 3) => '4',
            (2, 0) => '5',
            (2, 1) => '6',
            (2, 2) => '7',
            (2, 3) => '8',
            (2, 4) => '9',
            (3, 1) => 'A',
            (3, 2) => 'B',
            (3, 3) => 'C',
            (4, 2) => 'D',
            _ => bail!("Invalid position: ({}, {})", self.x, self.y),
        })
    }

    fn walk(&self, direction: Direction) -> Result<Position, Error> {
        let (x, y) = match direction {
            Up => (self.x - 1, self.y),
            Down => (self.x + 1, self.y),
            Left => (self.x, self.y - 1),
            Right => (self.x, self.y + 1),
        };
        let potential_position = Position { x, y };
        Ok(if potential_position.value().is_err() {
            *self
        } else {
            potential_position
        })
    }
}


fn read_input() -> Result<Vec<Vec<Direction>>, Error> {
    let stdin = io::stdin();
    let mut result = Vec::new();
    for line in stdin.lock().lines() {
        let line = line?;
        let line = line.trim();
        if !line.is_empty() {
            let mut directions = Vec::new();
            for ch in line.chars() {
                directions.push(ch.to_string().parse()?);
            }
            result.push(directions);
        }
    }
    Ok(result)
}


fn run() -> Result<(), Error> {
    let all_directions = read_input()?;

    let mut code = Vec::new();
    let mut position = Position::new();
    for directions in all_directions {
        for direction in directions {
            position = position.walk(direction)?;
        }
        code.push(position.value()?);
    }

    println!(
        "{}",
        code.iter()
            .map(|d| d.to_string())
            .collect::<Vec<_>>()
            .join("")
    );

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
