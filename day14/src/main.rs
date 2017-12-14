#[macro_use]
extern crate enum_map;
#[macro_use]
extern crate error_chain;
#[macro_use]
extern crate if_chain;

use enum_map::EnumMap;
use std::fmt::{self, Write};
use std::io;
use std::num::Wrapping;

error_chain! {
    foreign_links {
        Fmt(fmt::Error);
        Io(io::Error);
    }
}

fn reverse(map: &mut EnumMap<u8, u8>, start: u8, length: u8) {
    let mut positions = u32::from(start)..(u32::from(start) + u32::from(length));
    while let (Some(front), Some(back)) = (positions.next(), positions.next_back()) {
        map.swap(front as u8, back as u8);
    }
}

fn knot_hash(input: &[u8]) -> [u8; 16] {
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

fn flood_fill(input: &mut [[u8; 16]; 128], x: usize, y: usize) -> bool {
    let x_mask = 1 << (!x & 0b111);
    let x_pos = x >> 3;
    if_chain! {
        if let Some(&y_arr) = input.get(y);
        if let Some(&to_flood) = y_arr.get(x_pos);
        if to_flood & x_mask != 0;
        then {
            input[y][x_pos] = to_flood & !x_mask;
            for &(x, y) in &[
                (x.wrapping_sub(1), y),
                (x + 1, y),
                (x, y.wrapping_sub(1)),
                (x, y + 1),
            ] {
                flood_fill(input, x, y);
            }
            return true;
        }
    }
    false
}

fn run() -> Result<()> {
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    let new_len = input.trim_right().len();
    input.truncate(new_len);
    input += "-";
    let len_to_dash = input.len();
    let mut grid = [[0; 16]; 128];
    let mut used_squares = 0;
    for (number, output) in grid.iter_mut().enumerate() {
        input.truncate(len_to_dash);
        write!(input, "{}", number)?;
        *output = knot_hash(input.as_bytes());
        used_squares += output.iter().map(|number| number.count_ones()).sum::<u32>();
    }
    println!("Part 1: {}", used_squares);
    let mut groups = 0;
    for y in 0..128 {
        for x in 0..128 {
            groups += flood_fill(&mut grid, x, y) as u32;
        }
    }
    println!("Part 2: {}", groups);
    Ok(())
}

quick_main!(run);
