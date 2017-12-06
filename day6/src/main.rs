#[macro_use]
extern crate error_chain;

use std::collections::HashSet;
use std::io::{self, Read};
use std::num::ParseIntError;
use std::str::{self, Utf8Error};

error_chain! {
    foreign_links {
        Io(io::Error);
        ParseInt(ParseIntError);
        Utf8(Utf8Error);
    }
}

fn redistribute_memory(memory: &mut [u8]) -> usize {
    if memory.is_empty() {
        return 0;
    }
    let mut found = HashSet::new();
    while found.insert(memory.to_owned()) {
        let start_with;
        let iterations;
        {
            let (index, block) = memory
                .iter_mut()
                .enumerate()
                .max_by_key(|&(index, &mut block)| (block, !index))
                .unwrap();
            start_with = index + 1;
            iterations = *block;
            *block = 0;
        }
        let memory_len = memory.len() as u8;
        let add_everywhere = iterations / memory_len;
        let add_up_to = iterations % memory_len;
        let (second, first) = memory.split_at_mut(start_with);
        for (i, current_memory) in first.iter_mut().chain(second).enumerate() {
            let extra = ((i as u8) < add_up_to) as u8;
            *current_memory += add_everywhere + extra;
        }
    }
    found.len()
}

fn run() -> Result<()> {
    let mut input = Vec::new();
    io::stdin().read_to_end(&mut input)?;
    let blocks = &mut str::from_utf8(&input)?
        .split_whitespace()
        .map(|number| Ok(number.parse()?))
        .collect::<Result<Vec<u8>>>()?;
    println!("Day 1: {}", redistribute_memory(blocks));
    println!("Day 2: {}", redistribute_memory(blocks));
    Ok(())
}

quick_main!(run);

#[cfg(test)]
mod test {
    use redistribute_memory;

    #[test]
    fn test_redistribution() {
        let mut input = [0, 2, 7, 0];
        assert_eq!(redistribute_memory(&mut input), 5);
        assert_eq!(input, [2, 4, 1, 2]);
    }

}
