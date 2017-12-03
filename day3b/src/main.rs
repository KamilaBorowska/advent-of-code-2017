#[macro_use]
extern crate error_chain;
extern crate num_complex;

use num_complex::Complex;
use std::collections::HashMap;
use std::io;

error_chain! {
    foreign_links {
        Io(io::Error);
        ParseInt(std::num::ParseIntError);
    }
}

struct Spiral {
    current: Complex<i32>,
    direction: Complex<i32>,
}

impl Spiral {
    fn new() -> Spiral {
        Spiral {
            current: Complex::new(0, 0),
            direction: Complex::new(1, 0),
        }
    }

    fn next(&mut self) -> Complex<i32> {
        let quadrant_4 = self.current.re >= 0 && self.current.im <= 0;
        if !quadrant_4 && self.current.re.abs() == self.current.im.abs()
            || quadrant_4 && self.current.re - 1 == -self.current.im
        {
            self.direction *= Complex::i();
        }
        self.current += self.direction;
        self.current
    }
}

struct Square {
    positions: HashMap<Complex<i32>, u32>,
    spiral: Spiral,
}

impl Square {
    fn new() -> Square {
        let mut positions = HashMap::new();
        positions.insert(Complex::new(0, 0), 1);
        Square {
            positions,
            spiral: Spiral::new(),
        }
    }

    fn neighbor_sum(&self, position: Complex<i32>) -> u32 {
        let neighbors = [
            Complex::new(-1, -1),
            Complex::new(-1, 0),
            Complex::new(-1, 1),
            Complex::new(0, -1),
            Complex::new(0, 1),
            Complex::new(1, -1),
            Complex::new(1, 0),
            Complex::new(1, 1),
        ];
        neighbors
            .iter()
            .filter_map(|neighbor| self.positions.get(&(position + neighbor)))
            .sum()
    }
}

impl Iterator for Square {
    type Item = u32;

    fn next(&mut self) -> Option<u32> {
        let current = self.spiral.next();
        let sum = self.neighbor_sum(current);
        self.positions.insert(current, sum);
        Some(sum)
    }
}

fn run() -> Result<()> {
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    let input = input.trim().parse()?;
    println!("{}", Square::new().find(|&x| x > input).unwrap());
    Ok(())
}

quick_main!(run);

#[cfg(test)]
mod test {
    use Square;

    #[test]
    fn test_square() {
        let first_ten = [1, 2, 4, 5, 10, 11, 23, 25, 26, 54];
        assert_eq!(Square::new().take(10).collect::<Vec<_>>(), first_ten);
    }
}
