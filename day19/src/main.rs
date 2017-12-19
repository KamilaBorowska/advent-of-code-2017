extern crate num_complex;

use num_complex::Complex;
use std::io::{self, Read};

#[derive(Debug)]
struct Grid<'a> {
    input: &'a [u8],
    current_position: Complex<i32>,
    direction: Complex<i32>,
    width: usize,
}

impl<'a> Grid<'a> {
    fn new(input: &[u8]) -> Grid {
        let first_vertical_position = input
            .iter()
            .position(|&ch| ch == b'|')
            .expect("No vertical lines in image") as i32;
        let current_position = Complex::new(first_vertical_position, 0);
        let width = input
            .iter()
            .position(|&ch| ch == b'\n')
            .unwrap_or(input.len()) + 1;
        let direction = Complex::new(0, 1);
        Grid {
            input,
            current_position,
            direction,
            width,
        }
    }

    fn go(&mut self, x: i32, y: i32) -> bool {
        let direction = self.direction * Complex::new(x, y);
        let new_position = self.current_position + direction;
        let found_tile = self.at(new_position) != b' ';
        if found_tile {
            self.direction = direction;
            self.current_position = new_position;
        }
        found_tile
    }

    fn at(&self, position: Complex<i32>) -> u8 {
        let y_start = position.im as usize * self.width;
        self.input
            .get(y_start..y_start + self.width)
            .and_then(|slice| slice.get(position.re as usize))
            .cloned()
            .unwrap_or(b' ')
    }
}

fn find_letters(input: &[u8]) -> (Vec<u8>, u32) {
    let mut grid = Grid::new(input);
    let mut found = Vec::new();
    let mut steps = 1;
    while grid.go(1, 0) || grid.go(0, 1) || grid.go(0, -1) {
        let c = grid.at(grid.current_position);
        if !b"-+|".contains(&c) {
            found.push(c);
        }
        steps += 1;
    }
    (found, steps)
}

fn main() {
    let mut maze = Vec::new();
    if let Err(e) = io::stdin().read_to_end(&mut maze) {
        eprintln!("cannot read stdin: {}", e);
        return;
    }
    let (letters, steps) = find_letters(&maze);
    println!("Part 1: {}", String::from_utf8_lossy(&letters));
    println!("Part 2: {}", steps);
}

#[cfg(test)]
mod test {
    #[test]
    fn maze() {
        use find_letters;
        let maze = b"     |          
     |  +--+    
     A  |  C    
 F---|----E|--+ 
     |  |  |  D 
     +B-+  +--+ 
";
        assert_eq!(find_letters(maze), b"ABCDEF");
    }
}
