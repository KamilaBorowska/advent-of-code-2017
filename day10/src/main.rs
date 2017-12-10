#[macro_use]
extern crate enum_map;
#[macro_use]
extern crate error_chain;

use enum_map::EnumMap;
use std::io;
use std::num::{ParseIntError, Wrapping};

error_chain! {
    foreign_links {
        Io(io::Error);
        ParseInt(ParseIntError);
    }
}

fn reverse(map: &mut EnumMap<u8, u8>, start: u8, length: u8) {
    let mut positions = u32::from(start)..(u32::from(start) + u32::from(length));
    while let (Some(front), Some(back)) = (positions.next(), positions.next_back()) {
        map.swap(front as u8, back as u8);
    }
}

fn run() -> Result<()> {
    let mut line = String::new();
    io::stdin().read_line(&mut line)?;
    let mut array = enum_map! { i => i };
    let mut current_position = Wrapping(0);
    let mut skip = Wrapping(0);
    for length in line.trim().split(',') {
        let length = length.parse()?;
        reverse(&mut array, current_position.0, length);
        current_position += Wrapping(length) + skip;
        skip += Wrapping(1);
    }
    println!("Part 1: {}", u32::from(array[0]) * u32::from(array[1]));
    Ok(())
}

quick_main!(run);
