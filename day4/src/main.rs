#[macro_use]
extern crate error_chain;

use std::collections::HashSet;
use std::io::{self, BufRead};

error_chain! {
    foreign_links {
        Io(io::Error);
    }
}

fn contains_no_duplicate_words(line: &str) -> bool {
    let mut words_so_far = HashSet::new();
    for word in line.split_whitespace() {
        if !words_so_far.insert(word) {
            return false;
        }
    }
    true
}
fn run() -> Result<()> {
    let stdin = io::stdin();
    let mut line_count = 1;
    for line in stdin.lock().lines() {
        let line = line?;
        if !line.is_empty() && contains_no_duplicate_words(&line) {
            line_count += 1;
        }
    }
    println!("{}", line_count);
    Ok(())
}

quick_main!(run);

#[cfg(test)]
mod test {
    use contains_no_duplicate_words;

    #[test]
    fn test_basic_examples() {
        assert!(contains_no_duplicate_words("aa bb cc dd ee"));
        assert!(!contains_no_duplicate_words("aa bb cc dd aa"));
        assert!(contains_no_duplicate_words("aa bb cc dd aaa"));
    }
}
