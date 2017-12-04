#[macro_use]
extern crate error_chain;

use std::collections::HashSet;
use std::hash::Hash;
use std::io::{self, BufRead};

error_chain! {
    foreign_links {
        Io(io::Error);
    }
}

fn contains_no_duplicate<'a, F, T>(line: &'a str, mut f: F) -> bool
where
    F: FnMut(&'a str) -> T,
    T: 'a + Hash + Eq,
{
    let mut words_so_far = HashSet::new();
    for word in line.split_whitespace() {
        if !words_so_far.insert(f(word)) {
            return false;
        }
    }
    true
}

fn contains_no_duplicate_words(line: &str) -> bool {
    contains_no_duplicate(line, |word| word)
}

fn contains_no_duplicate_sorted_words(line: &str) -> bool {
    contains_no_duplicate(line, |word| {
        let mut sorted_word: Vec<char> = word.chars().collect();
        sorted_word.sort();
        sorted_word
    })
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
