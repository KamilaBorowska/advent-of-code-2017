use std::io;
use thiserror::Error;

#[derive(Error, Debug)]
enum Error {
    #[error("invalid character found")]
    InvalidCharacter,
    #[error("IO error: {0}")]
    Io(
        #[from]
        #[source]
        io::Error,
    ),
}

fn solve_captcha(input: &str, skip: usize) -> Result<u32, Error> {
    input
        .chars()
        .cycle()
        .skip(skip)
        .zip(input.chars())
        .filter(|&(a, b)| a == b)
        .map(|(a, _)| a.to_digit(10))
        .fold(Some(0), |a, b| Some(a? + b?))
        .ok_or(Error::InvalidCharacter)
}

fn solve_captcha_part1(input: &str) -> Result<u32, Error> {
    solve_captcha(input, 1)
}

fn solve_captcha_part2(input: &str) -> Result<u32, Error> {
    solve_captcha(input, input.len() / 2)
}

fn main() -> Result<(), Error> {
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    let input = input.trim();
    println!("Day 1 solution: {}", solve_captcha_part1(input)?);
    println!("Day 2 solution: {}", solve_captcha_part2(input)?);
    Ok(())
}

#[cfg(test)]
mod test {
    use crate::{solve_captcha_part1, solve_captcha_part2};

    fn test(input: &str, day1: u32, day2: u32) {
        assert_eq!(solve_captcha_part1(input).unwrap(), day1, "day1({})", input);
        assert_eq!(solve_captcha_part2(input).unwrap(), day2, "day2({})", input);
    }

    #[test]
    fn test_captcha_solving() {
        test("1122", 3, 0);
        test("1212", 0, 6);
        test("1221", 3, 0);
        test("1111", 4, 4);
        test("1234", 0, 0);
        test("123425", 0, 4);
        test("123123", 0, 12);
        test("12131415", 0, 4);
        test("91212129", 9, 6);
        test("", 0, 0);
    }

    #[test]
    fn test_failed_captcha_solving() {
        assert!(matches!(
            solve_captcha_part1("a"),
            Err(crate::Error::InvalidCharacter),
        ));
    }
}
