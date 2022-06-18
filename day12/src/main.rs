#[macro_use]
extern crate error_chain;
#[macro_use]
extern crate nom;

use std::collections::HashSet;
use std::io::{self, BufRead};
use std::str;
use std::usize;

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
struct Pipe {
    identifier: usize,
    connected_to: Vec<usize>,
}

named!(
    integer<usize>,
    map_res!(
        map_res!(take_while1!(nom::is_digit), str::from_utf8),
        str::parse
    )
);

named!(
    pipe<Pipe>,
    ws!(do_parse!(
        identifier: integer
            >> tag!("<->")
            >> connected_to: separated_nonempty_list_complete!(tag!(","), ws!(integer))
            >> (Pipe {
                identifier,
                connected_to,
            })
    ))
);

fn find_all(pipes: &[Pipe], pipe: usize, set: &mut HashSet<usize>) {
    set.insert(pipe);
    for &pipe in &pipes[pipe].connected_to {
        if set.insert(pipe) {
            find_all(pipes, pipe, set);
        }
    }
}

fn run() -> Result<()> {
    let stdin = io::stdin();
    let mut pipes = Vec::new();
    for line in stdin.lock().lines() {
        let pipe = pipe(line?.as_bytes())
            .to_result()
            .map_err(|_| ErrorKind::InvalidFormat)?;
        pipes.push(pipe);
    }
    let mut found = HashSet::new();
    let mut group_count = 0;
    for i in 0..pipes.len() {
        if found.contains(&i) {
            continue;
        }
        find_all(&pipes, i, &mut found);
        if i == 0 {
            println!("Part 1: {}", found.len());
        }
        group_count += 1;
    }
    println!("Part 2: {}", group_count);
    Ok(())
}

quick_main!(run);
