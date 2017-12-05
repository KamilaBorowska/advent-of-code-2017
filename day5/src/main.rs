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

fn steps<F>(jumps: &mut [isize], mut new_pc: F) -> u64
where
    F: FnMut(isize) -> isize,
{
    let mut pc = 0;
    let mut steps = 0;
    while let Some(add_pc) = jumps.get_mut(pc as usize) {
        pc += *add_pc;
        *add_pc = new_pc(*add_pc);
        steps += 1;
    }
    steps
}

fn steps_part1(jumps: &mut [isize]) -> u64 {
    steps(jumps, |pc| pc + 1)
}

fn steps_part2(jumps: &mut [isize]) -> u64 {
    steps(jumps, |pc| if pc >= 3 { pc - 1 } else { pc + 1 })
}

fn run() -> Result<()> {
    let stdin = io::stdin();
    let mut jumps = stdin
        .lock()
        .lines()
        .map(|line| Ok(line?.parse()?))
        .collect::<Result<Vec<isize>>>()?;
    println!("Part 1: {}", steps_part1(&mut jumps.clone()));
    println!("Part 2: {}", steps_part2(&mut jumps));
    Ok(())
}

quick_main!(run);
