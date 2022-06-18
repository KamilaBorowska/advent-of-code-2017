#[macro_use]
extern crate error_chain;
extern crate hex2d;

use hex2d::{Coordinate, Direction};
use std::io;

error_chain! {
    errors {
        UnrecognizedDirection {
            description("unrecognized direction")
        }
    }

    foreign_links {
        Io(io::Error);
    }
}

fn name_to_direction(name: &str) -> Option<Direction> {
    use crate::Direction::*;
    Some(match name {
        "n" => YZ,
        "ne" => XZ,
        "se" => XY,
        "s" => ZY,
        "sw" => ZX,
        "nw" => YX,
        _ => return None,
    })
}

fn run() -> Result<()> {
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    let origin = Coordinate::new(0, 0);
    let mut current_position = origin;
    let mut furthest = 0;
    for part in input.trim().split(',') {
        let direction = name_to_direction(part).ok_or(ErrorKind::UnrecognizedDirection)?;
        current_position = current_position + direction;
        furthest = origin.distance(current_position).max(furthest);
    }
    println!("Part 1: {}", origin.distance(current_position));
    println!("Part 2: {}", furthest);
    Ok(())
}

quick_main!(run);
