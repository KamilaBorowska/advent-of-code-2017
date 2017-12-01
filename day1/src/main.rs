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

fn solve_captcha(input: &str) -> Result<u32> {
    input
        .chars()
        .cycle()
        .skip(1)
        .zip(input.chars())
        .filter(|&(a, b)| a == b)
        .map(|(a, _)| a.to_digit(10))
        .fold(Some(0), |a, b| Some(a? + b?))
        .ok_or_else(|| ErrorKind::InvalidCharacter.into())
}

fn run() -> Result<()> {
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    println!("{}", solve_captcha(input.trim())?);
    Ok(())
}

quick_main!(run);

#[cfg(test)]
mod test {
    use solve_captcha;

    fn test(input: &str, output: u32) {
        assert_eq!(solve_captcha(input).unwrap(), output);
    }

    #[test]
    fn test_captcha_solving() {
        test("1122", 3);
        test("1111", 4);
        test("1234", 0);
        test("91212129", 9);
        test("", 0);
    }

    #[test]
    fn test_failed_captcha_solving() {
        use ErrorKind::InvalidCharacter;
        assert_matches!(solve_captcha("a").unwrap_err().kind(), &InvalidCharacter);
    }
}
