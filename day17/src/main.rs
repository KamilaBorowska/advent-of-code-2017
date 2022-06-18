extern crate itertools;

use std::cmp::Ordering;

// Workaround for conservative_impl_trait not being stable
macro_rules! positions {
    ($input:expr) => {
        itertools::iterate((0, 0), |&(i, current_position)| {
            (i + 1, (current_position + $input + 1) % (i + 1))
        })
    };
}

fn find_item(steps: usize, after: usize, input: usize) -> usize {
    assert!(steps >= after);
    let mut after_position = positions!(input)
        .find(|&(i, _)| i == after)
        .expect("after position not found")
        .1;
    let mut answer = 0;
    for (i, current_position) in positions!(input).take(steps) {
        match current_position.cmp(&after_position) {
            Ordering::Less if i > after => after_position += 1,
            Ordering::Equal => answer = i,
            _ => {}
        }
    }
    answer
}

fn main() {
    println!("Part 1: {}", find_item(2017, 2017, 344));
    println!("Part 2: {}", find_item(50_000_000, 0, 344));
}
