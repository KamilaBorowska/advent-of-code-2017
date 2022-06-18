#[macro_use]
extern crate nom;

use std::collections::HashMap;
use std::io::{self, Read};
use std::ops::AddAssign;
use std::str;

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

named!(
    point<Point>,
    ws!(do_parse!(
        tag!("<")
            >> x: integer
            >> tag!(",")
            >> y: integer
            >> tag!(",")
            >> z: integer
            >> tag!(">")
            >> (Point(x, y, z))
    ))
);

named!(
    particle<Particle>,
    ws!(do_parse!(
        tag!("p")
            >> tag!("=")
            >> position: point
            >> tag!(",")
            >> tag!("v")
            >> tag!("=")
            >> velocity: point
            >> tag!(",")
            >> tag!("a")
            >> tag!("=")
            >> acceleration: point
            >> (Particle {
                position,
                velocity,
                acceleration,
                exists: true,
            })
    ))
);

named!(particles<Vec<Particle>>, many1!(particle));

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
struct Point(i64, i64, i64);

impl AddAssign for Point {
    fn add_assign(&mut self, Point(x, y, z): Point) {
        self.0 += x;
        self.1 += y;
        self.2 += z;
    }
}

#[derive(Debug)]
struct Particle {
    position: Point,
    velocity: Point,
    acceleration: Point,
    exists: bool,
}

impl Particle {
    fn update(&mut self) {
        self.velocity += self.acceleration;
        self.position += self.velocity;
    }
}

fn main() {
    let mut input = Vec::new();
    io::stdin().read_to_end(&mut input).unwrap();
    let (_, mut particles) = particles(&input).unwrap();
    for _ in 0..500 {
        let mut found = HashMap::new();
        for i in 0..particles.len() {
            if particles[i].exists {
                if let Some(other) = found.insert(particles[i].position, i) {
                    particles[i].exists = false;
                    particles[other].exists = false;
                }
            }
            particles[i].update();
        }
    }

    let (part_1_position, _) = particles
        .iter()
        .enumerate()
        .min_by_key(
            |&(
                _,
                &Particle {
                    position: Point(x, y, z),
                    ..
                },
            )| x.abs() + y.abs() + z.abs(),
        )
        .unwrap();
    println!("Part 1: {:?}", part_1_position);
    println!(
        "Part 2: {:?}",
        particles.iter().filter(|particle| particle.exists).count()
    );
}
