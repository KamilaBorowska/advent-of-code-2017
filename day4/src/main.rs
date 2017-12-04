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

fn contains_no_duplicate_sorted_words(line: &str) -> bool {
    let mut words_so_far = HashSet::new();
    for word in line.split_whitespace() {
        let mut sorted_word: Vec<char> = word.chars().collect();
        sorted_word.sort();
        if !words_so_far.insert(sorted_word) {
            return false;
        }
    }
    return true;
}

fn run() -> Result<()> {
    let stdin = io::stdin();
    let mut no_duplicate_words = 0;
    let mut no_duplicate_sorted_words = 0;
    for line in stdin.lock().lines() {
        let line = line?;
        if contains_no_duplicate_words(&line) {
            no_duplicate_words += 1;
            if contains_no_duplicate_sorted_words(&line) {
                no_duplicate_sorted_words += 1;
            }
        }
    }
    println!("{} {}", no_duplicate_words, no_duplicate_sorted_words);
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
