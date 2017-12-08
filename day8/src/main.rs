#[macro_use]
extern crate error_chain;
#[macro_use]
extern crate nom;

use std::collections::HashMap;
use std::io::{self, Read};
use std::str::{self, Utf8Error};

struct Instruction<'a> {
    variable: &'a str,
    direction: Direction,
    change: i32,
    condvariable: &'a str,
    operator: Operator,
    condconst: i32,
}

impl<'a> Instruction<'a> {
    fn run(&self, variables: &mut Variables<'a>) {
        if self.operator.test(
            variables.map.get(self.condvariable).cloned().unwrap_or(0),
            self.condconst,
        ) {
            self.direction
                .change(variables.map.entry(self.variable).or_insert(0), self.change);
        }
    }
}

#[derive(Copy, Clone)]
enum Direction {
    Increment,
    Decrement,
}

impl Direction {
    fn change(self, value: &mut i32, change: i32) {
        match self {
            Direction::Increment => *value += change,
            Direction::Decrement => *value -= change,
        }
    }
}

#[derive(Copy, Clone)]
enum Operator {
    Ge,
    Le,
    Eq,
    Ne,
    Gt,
    Lt,
}

impl Operator {
    fn test(self, a: i32, b: i32) -> bool {
        match self {
            Operator::Ge => a >= b,
            Operator::Le => a <= b,
            Operator::Eq => a == b,
            Operator::Ne => a != b,
            Operator::Gt => a > b,
            Operator::Lt => a < b,
        }
    }
}

#[derive(Default)]
struct Variables<'a> {
    map: HashMap<&'a str, i32>,
}

impl<'a> Variables<'a> {
    fn new() -> Self {
        Variables::default()
    }
}

named!(instruction<&str, Instruction>, ws!(do_parse!(
    variable: identifier >>
    direction: alt!(
        tag_s!("inc") => { |_| Direction::Increment } |
        tag_s!("dec") => { |_| Direction::Decrement }
    ) >>
    change: integer >>
    tag_s!("if") >>
    condvariable: identifier >>
    operator: alt!(
        tag_s!(">=") => { |_| Operator::Ge } |
        tag_s!("<=") => { |_| Operator::Le } |
        tag_s!("==") => { |_| Operator::Eq } |
        tag_s!("!=") => { |_| Operator::Ne } |
        tag_s!(">") => { |_| Operator::Gt } |
        tag_s!("<") => { |_| Operator::Lt }
    ) >>
    condconst: integer >>
    (Instruction {variable, direction, change, condvariable, operator, condconst})
)));

named!(identifier<&str, &str>, take_while1_s!(char::is_alphabetic));

named!(integer<&str, i32>,  map_res!(take_while1_s!(|c| c == '-' || char::is_digit(c, 10)), str::parse));

error_chain! {
    errors {
        InvalidFormat {
            description("input format doesn't match expected")
        }
    }

    foreign_links {
        Io(io::Error);
        Utf8(Utf8Error);
    }
}

fn run() -> Result<()> {
    let mut input = Vec::new();
    io::stdin().read_to_end(&mut input)?;
    let mut variables = Variables::new();
    for line in str::from_utf8(&input)?.lines() {
        instruction(line)
            .to_result()
            .map_err(|_| ErrorKind::InvalidFormat)?
            .run(&mut variables);
    }
    println!(
        "Part 1: {}",
        variables.map.values().max().cloned().unwrap_or(0)
    );
    Ok(())
}

quick_main!(run);
