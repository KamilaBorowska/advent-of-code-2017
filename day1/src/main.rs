#[macro_use]
extern crate error_chain;
#[cfg(test)]
#[macro_use]
extern crate matches;

use std::io;

error_chain! {
    errors {
        InvalidCharacter {
            description("invalid character found")
        }
    }

    foreign_links {
        Io(io::Error);
    }
}

fn solve_captcha(input: &str, skip: usize) -> Result<u32> {
    input
        .chars()
        .cycle()
        .skip(skip)
        .zip(input.chars())
        .filter(|&(a, b)| a == b)
        .map(|(a, _)| a.to_digit(10))
        .fold(Some(0), |a, b| Some(a? + b?))
        .ok_or_else(|| ErrorKind::InvalidCharacter.into())
}

fn solve_captcha_part1(input: &str) -> Result<u32> {
    solve_captcha(input, 1)
}

fn solve_captcha_part2(input: &str) -> Result<u32> {
    solve_captcha(input, input.len() / 2)
}

fn run() -> Result<()> {
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    let input = input.trim();
    println!("Day 1 solution: {}", solve_captcha_part1(input)?);
    println!("Day 2 solution: {}", solve_captcha_part2(input)?);
    Ok(())
}

quick_main!(run);

#[cfg(test)]
mod test {
    use {solve_captcha_part1, solve_captcha_part2};

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
        use ErrorKind::InvalidCharacter;
        assert_matches!(
            solve_captcha_part1("a").unwrap_err().kind(),
            &InvalidCharacter
        );
    }
}
