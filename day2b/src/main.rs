#[macro_use]
extern crate error_chain;
extern crate itertools;

use itertools::Itertools;
use std::io::{self, BufRead};
use std::num::ParseIntError;
use std::result;

error_chain! {
    errors {
        NoEvenlyDivisibleNumber {
            description("no evenly divisible number")
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
        let numbers: result::Result<Vec<u32>, _> =
            line?.split_whitespace().map(str::parse).collect();
        checksum += line_checksum(&numbers?)?;
    }
    Ok(checksum)
}

fn line_checksum(numbers: &[u32]) -> Result<u32> {
    numbers
        .iter()
        .enumerate()
        .cartesian_product(numbers.iter().enumerate())
        .find(|&((i, a), (j, b))| i != j && a % b == 0)
        .map(|((_, a), (_, b))| a / b)
        .ok_or_else(|| ErrorKind::NoEvenlyDivisibleNumber.into())
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
        let spreadsheet = b"5 9 2 8\n9 4 7 3\n3 8 6 5";
        assert_eq!(checksum(&spreadsheet[..]).unwrap(), 9);
    }
}
