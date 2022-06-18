#[macro_use]
extern crate error_chain;
extern crate itertools;

use itertools::Itertools;
use std::io::{self, BufRead};
use std::num::ParseIntError;

error_chain! {
    errors {
        EmptyLine {
            description("empty line retrieved: they have neither minimal nor maximal value")
        }
    }

    foreign_links {
        Io(io::Error);
        ParseInt(ParseIntError);
    }
}

fn checksum<T: BufRead>(input: T) -> Result<u32> {
    let mut checksum = 0;
    for line in input.lines() {
        let (min, max) = line?
            .split_whitespace()
            .map(str::parse::<u32>)
            .minmax_by_key(|result| result.as_ref().ok().cloned())
            .into_option()
            .ok_or(ErrorKind::EmptyLine)?;
        checksum += max? - min?;
    }
    Ok(checksum)
}

fn run() -> Result<()> {
    let stdin = io::stdin();
    println!("{}", checksum(stdin.lock())?);
    Ok(())
}

quick_main!(run);

#[cfg(test)]
mod test {
    use crate::checksum;

    #[test]
    fn test_checksum() {
        let spreadsheet = b"5 1 9 5\n7 5 3\n2 4 6 8\n";
        assert_eq!(checksum(&spreadsheet[..]).unwrap(), 18);
    }
}
