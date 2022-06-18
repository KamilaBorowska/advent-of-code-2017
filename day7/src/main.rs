#[macro_use]
extern crate error_chain;
#[macro_use]
extern crate nom;

use std::collections::HashSet;
use std::io::{self, Read};
use std::str::{self, Utf8Error};

error_chain! {
    errors {
        BottomNotFound {
            description("bottom element doesn't exist")
        }
        InvalidFormat {
            description("input format doesn't match expected")
        }
    }

    foreign_links {
        Io(io::Error);
        Utf8(Utf8Error);
    }
}

#[derive(Debug, PartialEq, Eq)]
struct Line<'a> {
    label: &'a str,
    weight: u32,
    parts: Vec<&'a str>,
}

named!(lines<&str, Vec<Line> >, many0!(terminated!(line, nom::line_ending)));

named!(line<&str, Line>, sep!(space, do_parse!(
    label: identifier >>
    weight: delimited!(tag_s!("("), integer, tag_s!(")")) >>
    parts: opt!(preceded!(tag_s!("->"), separated_list!(tag_s!(","), sep!(space, identifier)))) >>
    (Line { label, weight, parts: parts.unwrap_or_default() })
)));

named!(space<&str, ()>, do_parse!(many0!(tag_s!(" ")) >> ()));

named!(identifier<&str, &str>, take_while1_s!(char::is_alphabetic));

named!(integer<&str, u32>,  map_res!(take_while1_s!(|c| char::is_digit(c, 10)), str::parse));

fn find_bottom<'a>(lines: &'a [Line<'a>]) -> Option<&'a Line<'a>> {
    let found: HashSet<&str> = lines.iter().flat_map(|line| &line.parts).cloned().collect();
    lines.iter().find(|line| !found.contains(line.label))
}

fn run() -> Result<()> {
    let mut input = Vec::new();
    io::stdin().read_to_end(&mut input)?;
    let lines = lines(str::from_utf8(&input)?)
        .to_result()
        .map_err(|_| ErrorKind::InvalidFormat)?;
    let bottom = find_bottom(&lines).ok_or(ErrorKind::BottomNotFound)?;
    println!("Part 1: {}", bottom.label);
    Ok(())
}

quick_main!(run);

#[cfg(test)]
mod test {
    use nom::IResult;
    use {identifier, integer, line, Line};

    #[test]
    fn test_identifier() {
        assert_eq!(identifier("hello+"), IResult::Done("+", "hello"));
    }

    #[test]
    fn test_integer() {
        assert_eq!(integer("42+"), IResult::Done("+", 42));
    }

    #[test]
    fn test_line_parsing() {
        assert_eq!(
            line("a(42)\n"),
            IResult::Done(
                "\n",
                Line {
                    label: "a",
                    weight: 42,
                    parts: Vec::new(),
                }
            )
        );
    }

    #[test]
    fn test_line_parsing_with_parts() {
        assert_eq!(
            line("a ( 42 ) -> b , c\n"),
            IResult::Done(
                "\n",
                Line {
                    label: "a",
                    weight: 42,
                    parts: vec!["b", "c"],
                }
            )
        )
    }
}
