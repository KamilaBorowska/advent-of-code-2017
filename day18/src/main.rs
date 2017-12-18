#[macro_use]
extern crate error_chain;
#[macro_use]
extern crate nom;

use nom::be_u8;
use std::collections::VecDeque;
use std::io::{self, Read};
use std::str;

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

#[derive(Debug)]
struct Interpreter<'a> {
    program: &'a [Opcode],
    registers: [i64; 26],
    current_position: usize,
}

impl<'a> Interpreter<'a> {
    fn new(program: &'a [Opcode]) -> Self {
        Interpreter {
            program,
            registers: [0; 26],
            current_position: 0,
        }
    }

    fn step(&mut self) -> Command {
        if self.current_position == self.program.len() {
            return Command::Eof;
        }
        let previous_position = self.current_position;
        self.current_position += 1;
        match self.program[previous_position] {
            Opcode::Snd(value) => return Command::Send(self.read_value(value)),
            Opcode::Set(register, value) => {
                let value = self.read_value(value);
                self.write_register(register, value);
            }
            Opcode::Add(register, value) => self.mutate_using(register, value, |a, b| a + b),
            Opcode::Mul(register, value) => self.mutate_using(register, value, |a, b| a * b),
            Opcode::Mod(register, value) => self.mutate_using(register, value, |a, b| a % b),
            Opcode::Rcv(register) => return Command::Receive(self.register_mut(register)),
            Opcode::Jgz(value, offset) => {
                if self.read_value(value) > 0 {
                    self.current_position =
                        (previous_position as i64 + self.read_value(offset)) as usize;
                }
            }
        }
        Command::Regular
    }

    fn read_value(&self, value: Value) -> i64 {
        match value {
            Value::Register(register) => self.read_register(register),
            Value::Constant(constant) => constant,
        }
    }

    fn mutate_using<F>(&mut self, register: Register, value: Value, callback: F)
    where
        F: FnOnce(i64, i64) -> i64,
    {
        let result = callback(self.read_register(register), self.read_value(value));
        self.write_register(register, result);
    }

    fn read_register(&self, Register(name): Register) -> i64 {
        self.registers[usize::from(name - b'a')]
    }

    fn write_register(&mut self, register: Register, value: i64) {
        *self.register_mut(register) = value;
    }

    fn register_mut(&mut self, Register(name): Register) -> &mut i64 {
        &mut self.registers[usize::from(name - b'a')]
    }
}

enum Command<'a> {
    Send(i64),
    Receive(&'a mut i64),
    Regular,
    Eof,
}

#[derive(Copy, Clone, Debug)]
enum Opcode {
    Snd(Value),
    Set(Register, Value),
    Add(Register, Value),
    Mul(Register, Value),
    Mod(Register, Value),
    Rcv(Register),
    Jgz(Value, Value),
}

#[derive(Copy, Clone, Debug)]
enum Value {
    Register(Register),
    Constant(i64),
}

#[derive(Copy, Clone, Debug)]
struct Register(u8);

named!(instructions<Vec<Opcode>>, complete!(many0!(instruction)));
named!(instruction<Opcode>, ws!(alt!(snd | rcv | arithm | jgz)));
named!(
    snd<Opcode>,
    ws!(preceded!(tag!("snd"), map!(value, Opcode::Snd)))
);
named!(
    rcv<Opcode>,
    ws!(preceded!(tag!("rcv"), map!(register, Opcode::Rcv)))
);
named!(
    arithm<Opcode>,
    ws!(do_parse!(
        opcode:
            alt!(
        tag!("set") => { |_| Opcode::Set as fn(_, _) -> _ } |
        tag!("add") => { |_| Opcode::Add as fn(_, _) -> _ } |
        tag!("mul") => { |_| Opcode::Mul as fn(_, _) -> _ } |
        tag!("mod") => { |_| Opcode::Mod as fn(_, _) -> _ }
    ) >> register: register >> value: value >> (opcode(register, value))
    ))
);
named!(
    jgz<Opcode>,
    ws!(do_parse!(
        tag!("jgz") >> check: value >> offset: value >> (Opcode::Jgz(check, offset))
    ))
);
named!(
    value<Value>,
    alt!(integer => { |i| Value::Constant(i) } | register => {|r| Value::Register(r)})
);
named!(register<Register>, map!(be_u8, Register));
named!(
    integer<i64>,
    map_res!(
        map_res!(
            take_while1!(|c| c == b'-' || char::is_digit(char::from(c), 10)),
            str::from_utf8
        ),
        str::parse
    )
);

fn part1(instructions: &[Opcode]) -> i64 {
    let mut interpreter = Interpreter::new(instructions);
    let mut last_sound = 0;
    loop {
        match interpreter.step() {
            Command::Send(value) => last_sound = value,
            Command::Receive(_) => if last_sound != 0 {
                return last_sound;
            },
            Command::Eof => return 0,
            _ => {}
        }
    }
}

fn part2(instructions: &[Opcode]) -> u32 {
    let mut interpreter0 = Interpreter::new(instructions);
    let mut interpreter1 = Interpreter::new(instructions);
    interpreter1.write_register(Register(b'p'), 1);
    let mut sent_to_interpreter0 = VecDeque::new();
    let mut values_sent = 0;
    'outerloop: loop {
        match interpreter1.step() {
            Command::Send(value) => {
                values_sent += 1;
                sent_to_interpreter0.push_front(value);
            }
            Command::Receive(value) => {
                *value = loop {
                    match interpreter0.step() {
                        Command::Send(value) => break value,
                        Command::Receive(out) => match sent_to_interpreter0.pop_back() {
                            Some(value) => *out = value,
                            None => break 'outerloop,
                        },
                        Command::Eof => break 'outerloop,
                        _ => {}
                    }
                }
            }
            Command::Eof => break,
            _ => {}
        }
    }
    values_sent
}

fn run() -> Result<()> {
    let mut input = Vec::new();
    io::stdin().read_to_end(&mut input)?;
    let instructions = &instructions(&input)
        .to_result()
        .map_err(|_| ErrorKind::InvalidFormat)?;
    println!("Part 1: {}", part1(instructions));
    println!("Part 2: {}", part2(instructions));
    Ok(())
}

quick_main!(run);
