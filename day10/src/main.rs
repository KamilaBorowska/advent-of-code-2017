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

fn part_one(line: &str) -> Result<u32> {
    let mut array = enum_map! { i => i };
    let mut current_position = Wrapping(0);
    let mut skip = Wrapping(0);
    for length in line.split(',') {
        let length = length.parse()?;
        reverse(&mut array, current_position.0, length);
        current_position += Wrapping(length) + skip;
        skip += Wrapping(1);
    }
    Ok(u32::from(array[0]) * u32::from(array[1]))
}

fn part_two(input: &[u8]) -> [u8; 16] {
    let mut array = enum_map! { i => i };
    let mut current_position = Wrapping(0);
    let mut skip = Wrapping(0);
    for _round in 0..64 {
        for &byte in input.iter().chain(&[17, 31, 73, 47, 23]) {
            reverse(&mut array, current_position.0, byte);
            current_position += Wrapping(byte) + skip;
            skip += Wrapping(1);
        }
    }
    let mut output = [0; 16];
    for (chunk, out) in array.as_slice().chunks(16).zip(&mut output) {
        *out = chunk.iter().fold(0, |a, b| a ^ b);
    }
    output
}

fn run() -> Result<()> {
    let mut line = String::new();
    io::stdin().read_line(&mut line)?;
    println!("Part 1: {}", part_one(line.trim())?);
    print!("Part 2: ");
    for byte in &part_two(line.trim().as_bytes()) {
        print!("{:02x}", byte);
    }
    println!();
    Ok(())
}

quick_main!(run);
