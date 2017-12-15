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
    North,
    South,
    East,
    West,
}

use Direction::*;

#[derive(EnumString, Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
enum Turn {
    #[strum(serialize = "L")]
    Left,
    #[strum(serialize = "R")]
    Right,
}

use Turn::*;


impl Direction {
    fn turn(&self, turn: Turn) -> Direction {
        match (*self, turn) {
            (North, Left) => West,
            (North, Right) => East,
            (South, Left) => East,
            (South, Right) => West,
            (East, Left) => North,
            (East, Right) => South,
            (West, Left) => South,
            (West, Right) => North,
        }
    }
}


#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
struct Position {
    x: i32,
    y: i32,
}

impl Position {
    fn new() -> Position {
        Position { x: 0, y: 0 }
    }

    fn distance_to_origo(&self) -> i32 {
        self.x.abs() + self.y.abs()
    }

    fn walk(&self, direction: Direction, steps: i32) -> Position {
        let (x, y) = match direction {
            North => (self.x + steps, self.y),
            South => (self.x - steps, self.y),
            East => (self.x, self.y + steps),
            West => (self.x, self.y - steps),
        };
        Position { x, y }
    }
}


fn read_input() -> Result<Vec<(Turn, i32)>, Error> {
    let stdin = io::stdin();
    let input: String = stdin.lock().lines().next().unwrap()?;
    let mut result = Vec::new();
    for word in input.split(", ") {
        let turn = word.get(0..1).unwrap().parse()?;
        let steps = word.get(1..).unwrap().parse()?;
        result.push((turn, steps));
    }
    Ok(result)
}


fn run() -> Result<(), Error> {
    let instructions = read_input()?;

    let mut position = Position::new();
    let mut direction = North;

    for (turn, steps) in instructions {
        direction = direction.turn(turn);
        position = position.walk(direction, steps);
    }

    println!("{}", position.distance_to_origo());

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
