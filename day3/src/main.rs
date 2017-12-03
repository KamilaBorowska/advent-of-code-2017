#[macro_use]
extern crate error_chain;
extern crate integer_sqrt;

use integer_sqrt::IntegerSquareRoot;
use std::io;

error_chain! {
    foreign_links {
        Io(io::Error);
        ParseInt(std::num::ParseIntError);
    }
}

// From Math::PlanePath::SquareSpiral CPAN module by
// Kevin Ryde (http://user42.tuxfamily.org/), GPLv3 or later
fn position(n: u32) -> (i32, i32) {
    let d = ((2 + (4 * n).integer_sqrt()) / 4) as i32;
    let n = n as i32 - (4 * d * d);
    if n >= 0 {
        if n <= 2 * d {
            (-d, d - n)
        } else {
            (n - 3 * d, -d)
        }
    } else {
        if n >= -2 * d {
            (-n - d, d)
        } else {
            (d, n + 3 * d)
        }
    }
}

fn steps(n: u32) -> u32 {
    let (x, y) = position(n - 1);
    x.abs() as u32 + y.abs() as u32
}

fn run() -> Result<()> {
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    println!("Steps: {}", steps(input.trim().parse()?));
    Ok(())
}

quick_main!(run);

#[cfg(test)]
mod test {
    use steps;

    #[test]
    fn test_steps() {
        let tests = [
            (1, 0),
            (2, 1),
            (3, 2),
            (4, 1),
            (5, 2),
            (6, 1),
            (7, 2),
            (8, 1),
            (9, 2),
            (10, 3),
            (11, 2),
            (12, 3),
            (13, 4),
            (14, 3),
            (15, 2),
            (16, 3),
            (17, 4),
            (18, 3),
            (19, 2),
            (20, 3),
            (21, 4),
            (22, 3),
            (23, 2),
            (1024, 31),
        ];
        for &(n, expected) in &tests {
            assert_eq!(steps(n), expected, "steps({})", n)
        }
    }
}
