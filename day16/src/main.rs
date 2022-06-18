#[macro_use]
extern crate array_macro;
#[macro_use]
extern crate error_chain;
#[macro_use]
extern crate nom;

use nom::{be_u8, digit};
use std::io::{self, Read};
use std::str;

error_chain! {
    errors {
        InvalidFormat {
            description("input format doesn't match expected")
        }
        InvalidPartner {
            description("partner command asked to swap non-existing letters")
        }
    }

    foreign_links {
        Io(io::Error);
    }
}

enum Command {
    Spin(u8),
    Exchange(u8, u8),
    Partner(u8, u8),
}

named!(commands<Vec<Command>>, separated_list!(tag!(","), command));
named!(command<Command>, alt!(spin | exchange | partner));

named!(
    spin<Command>,
    map!(preceded!(tag!("s"), integer), Command::Spin)
);

named!(
    exchange<Command>,
    do_parse!(
        tag!("x")
            >> first: integer
            >> tag!("/")
            >> second: integer
            >> (Command::Exchange(first, second))
    )
);

named!(
    partner<Command>,
    do_parse!(
        tag!("p")
            >> first: be_u8
            >> tag!("/")
            >> second: be_u8
            >> (Command::Partner(first, second))
    )
);

named!(
    integer<u8>,
    map_res!(map_res!(digit, str::from_utf8), str::parse)
);

// slice rotation is not stable
fn rotate<T>(slice: &mut [T], skip: usize) {
    slice.reverse();
    let (a, b) = slice.split_at_mut(skip);
    a.reverse();
    b.reverse();
}

fn find_letter(dancing: &[u8], letter: u8) -> Result<usize> {
    Ok(dancing
        .iter()
        .position(|&another| letter == another)
        .ok_or(ErrorKind::InvalidPartner)?)
}

fn run() -> Result<()> {
    const ITERATIONS: usize = 1_000_000_000;

    let mut sequences = Vec::new();
    let mut dancing = array![i => b'a' + i as u8; 16];
    let mut input = Vec::new();
    io::stdin().read_to_end(&mut input)?;
    let commands = commands(&input)
        .to_result()
        .map_err(|_| ErrorKind::InvalidFormat)?;
    for i in 0..ITERATIONS {
        for command in &commands {
            match *command {
                Command::Spin(size) => rotate(&mut dancing, usize::from(size)),
                Command::Exchange(a, b) => dancing.swap(usize::from(a), usize::from(b)),
                Command::Partner(a, b) => {
                    let a_pos = find_letter(&dancing, a)?;
                    let b_pos = find_letter(&dancing, b)?;
                    dancing.swap(a_pos, b_pos);
                }
            }
        }
        if Some(&dancing) == sequences.get(0) {
            dancing = sequences[ITERATIONS % i - 1];
            break;
        }
        sequences.push(dancing);
        if i == 0 {
            println!("Part 1: {}", String::from_utf8_lossy(&dancing));
        }
    }
    println!("Part 2: {}", String::from_utf8_lossy(&dancing));
    Ok(())
}

quick_main!(run);
