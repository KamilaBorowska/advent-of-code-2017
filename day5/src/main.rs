#[macro_use]
extern crate error_chain;

use std::io::{self, BufRead};
use std::num::ParseIntError;

error_chain! {
    foreign_links {
        Io(io::Error);
        ParseInt(ParseIntError);
    }
}

fn run() -> Result<()> {
    let stdin = io::stdin();
    let mut jumps = stdin
        .lock()
        .lines()
        .map(|line| Ok(line?.parse()?))
        .collect::<Result<Vec<isize>>>()?;
    let mut pc = 0;
    let mut steps = 0;
    while let Some(add_pc) = jumps.get_mut(pc as usize) {
        pc += *add_pc;
        *add_pc += 1;
        steps += 1;
    }
    println!("{}", steps);
    Ok(())
}

quick_main!(run);
