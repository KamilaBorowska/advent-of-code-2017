#[macro_use]
extern crate error_chain;
#[macro_use]
extern crate nom;

use nom::line_ending;
use std::io::{self, Read};
use std::str;

error_chain! {
    errors {
        InvalidFormat {
            description("input format doesn't match expected")
        }
    }

    foreign_links {
        Io(io::Error);
    }
}

named!(
    generators<(Generator, Generator)>,
    do_parse!(
        tag!("Generator A starts with ")
            >> a: integer
            >> line_ending
            >> tag!("Generator B starts with ")
            >> b: integer
            >> (
                Generator {
                    current: a,
                    factor: 16_807,
                },
                Generator {
                    current: b,
                    factor: 48_271,
                }
            )
    )
);

named!(
    integer<u32>,
    map_res!(
        map_res!(take_while1!(nom::is_digit), str::from_utf8),
        str::parse
    )
);

#[derive(Clone)]
struct Generator {
    current: u32,
    factor: u32,
}

impl Iterator for Generator {
    type Item = u32;
    fn next(&mut self) -> Option<u32> {
        let current = u64::from(self.current) * u64::from(self.factor) % 2_147_483_647;
        self.current = current as u32;
        Some(self.current)
    }
}

fn run() -> Result<()> {
    let mut input = Vec::new();
    io::stdin().read_to_end(&mut input)?;
    let generators = generators(&input)
        .to_result()
        .map_err(|_| ErrorKind::InvalidFormat)?;
    let (a, b) = generators.clone();
    println!(
        "Part 1: {}",
        a.zip(b)
            .take(40_000_000)
            .filter(|&(a, b)| a as u16 == b as u16)
            .count()
    );
    let (a, b) = generators;
    println!(
        "Part 2: {}",
        a.filter(|a| a % 4 == 0)
            .zip(b.filter(|b| b % 8 == 0))
            .take(5_000_000)
            .filter(|&(a, b)| a as u16 == b as u16)
            .count()
    );
    Ok(())
}

quick_main!(run);
