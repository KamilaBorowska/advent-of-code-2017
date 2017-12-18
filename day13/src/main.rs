#[macro_use]
extern crate error_chain;
#[macro_use]
extern crate nom;

use std::io::{self, BufRead};
use std::str;

error_chain! {
    errors {
        InvalidFormat {
            description("input format doesn't match expected")
        }
        NoDelayWithoutBeingCaught {
            description("there is no delay on which user isn't caught")
        }
    }

    foreign_links {
        Io(io::Error);
    }
}

struct Firewall {
    depth: u32,
    range: u32,
}

impl Firewall {
    fn cycle_length(&self) -> u32 {
        if self.range < 2 {
            return 1;
        }
        2 * self.range - 2
    }
}

named!(
    integer<u32>,
    map_res!(
        map_res!(take_while1!(nom::is_digit), str::from_utf8),
        str::parse
    )
);

named!(
    firewall<Firewall>,
    ws!(do_parse!(
        depth: integer >> tag!(":") >> range: integer >> (Firewall { depth, range })
    ))
);

fn severity(firewalls: &[Firewall]) -> u32 {
    firewalls
        .iter()
        .filter(|firewall| firewall.depth % firewall.cycle_length() == 0)
        .map(|firewall| firewall.depth * firewall.range)
        .sum()
}

fn is_caught(firewalls: &[Firewall], delay: u32) -> bool {
    firewalls
        .iter()
        .any(|firewall| (firewall.depth + delay) % firewall.cycle_length() == 0)
}

fn run() -> Result<()> {
    let stdin = io::stdin();
    let mut security_scanners = Vec::new();
    for line in stdin.lock().lines() {
        let firewall = firewall(line?.as_bytes())
            .to_result()
            .map_err(|_| ErrorKind::InvalidFormat)?;
        security_scanners.push(firewall);
    }
    println!("Part 1: {}", severity(&security_scanners));
    println!(
        "Part 2: {}",
        (0..)
            .find(|&delay| !is_caught(&security_scanners, delay))
            .ok_or(ErrorKind::NoDelayWithoutBeingCaught)?
    );
    Ok(())
}

quick_main!(run);
