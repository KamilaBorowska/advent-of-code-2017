#[macro_use]
extern crate error_chain;
#[macro_use]
extern crate nom;

use std::io::{self, Read};

#[derive(Debug, PartialEq, Eq)]
enum Group<'a> {
    Braces(Vec<Group<'a>>),
    Garbage(&'a [u8]),
}

impl<'a> Group<'a> {
    fn score(&self, current_score: u32) -> u32 {
        if let Group::Braces(ref groups) = *self {
            current_score
                + groups
                    .iter()
                    .map(|group| group.score(current_score + 1))
                    .sum::<u32>()
        } else {
            0
        }
    }

    fn non_cancelled_characters(&self) -> u32 {
        match *self {
            Group::Braces(ref groups) => groups.iter().map(Group::non_cancelled_characters).sum(),
            Group::Garbage(garbage) => {
                let mut char_iter = garbage.iter();
                let mut count = 0;
                while let Some(&ch) = char_iter.next() {
                    if ch == b'!' {
                        char_iter.next();
                    } else {
                        count += 1;
                    }
                }
                count
            }
        }
    }
}

named!(
    group<Group>,
    alt!(map!(braces, Group::Braces) | map!(garbage, Group::Garbage))
);
named!(
    braces<Vec<Group>>,
    delimited!(tag!("{"), separated_list!(tag!(","), group), tag!("}"))
);
named!(
    garbage,
    delimited!(
        tag!("<"),
        escaped!(none_of!(">!"), '!', nom::anychar),
        tag!(">")
    )
);

error_chain! {
    errors {
        InvalidFormat {
            description("input format doesn't match expected")
        }
    }

    foreign_links {
        Io(io::Error);
    }
}

fn run() -> Result<()> {
    let mut input = Vec::new();
    io::stdin().read_to_end(&mut input)?;
    let group = group(&input)
        .to_result()
        .map_err(|_| ErrorKind::InvalidFormat)?;
    println!("Part 1: {}", group.score(1));
    println!("Part 2: {}", group.non_cancelled_characters());
    Ok(())
}

quick_main!(run);

#[cfg(test)]
mod test {
    use crate::{braces, garbage, Group};
    use nom::IResult::Done;

    #[test]
    fn test_garbage() {
        assert_eq!(garbage(b"<df>x"), Done("x".as_bytes(), "df".as_bytes()));
        assert_eq!(garbage(b"<d!f>x"), Done("x".as_bytes(), "d!f".as_bytes()));
        assert_eq!(garbage(b"<d!>f>x"), Done("x".as_bytes(), "d!>f".as_bytes()));
    }

    #[test]
    fn test_braces() {
        assert_eq!(braces(b"{}"), Done("".as_bytes(), Vec::new()));
        assert_eq!(
            braces(b"{<a>,{}}"),
            Done(
                "".as_bytes(),
                vec![Group::Garbage(b"a"), Group::Braces(Vec::new())]
            )
        );
    }
}
