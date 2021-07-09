use aoc_core::{AoCDay, ErrorWrapper, math};
use lazy_static::lazy_static;

type Num = isize;
type Vec3 = aoc_core::Vec3<Num>;
pub struct Day12;

const INPUT: &str = include_str!("../input/day_12.txt");

fn parse(input: &str) -> Vec<Body> {
    input.trim().lines().enumerate().map(|(i, l)| {
        let n: Vec<Num> = l.trim_matches(|c| c == '<' || c == '>').split(',').map(|a| a.trim()).map(|a| a[2..].parse().unwrap()).collect();
        Body::new(i, Vec3::new(n[0], n[1], n[2]))
    }).collect()
}

lazy_static! {
    static ref BODIES: Vec<Body> = parse(INPUT);
}

#[derive(Clone, Hash, PartialEq, Eq)]
struct Body {
    id: usize,
    position: Vec3,
    velocity: Vec3,
}

impl std::fmt::Display for Body {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "Body({})", self.id)
    }
}
impl std::fmt::Debug for Body {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "pos=<x={:3}, y={:3}, z={:3}>, vel=<x={:3}, y={:3}, z={:3}>", self.position.x, self.position.y, self.position.z, self.velocity.x, self.velocity.y, self.velocity.z)
    }
}

impl Body {
    fn new(id: usize, position: Vec3) -> Body {
        Body {
            id,
            position,
            velocity: Default::default(),
        }
    }

    fn update(&mut self) {
        self.position += self.velocity;
    }

    fn interact(&mut self, other: &mut Self) {
        use std::cmp::Ordering;
        match (self.position.x - other.position.x).cmp(&0) {
            Ordering::Greater => {
                self.velocity.x -= 1;
                other.velocity.x += 1;
            },
            Ordering::Less => {
                self.velocity.x += 1;
                other.velocity.x -= 1;
            },
            Ordering::Equal => (),
        }
        match (self.position.y - other.position.y).cmp(&0) {
            Ordering::Greater => {
                self.velocity.y -= 1;
                other.velocity.y += 1;
            },
            Ordering::Less => {
                self.velocity.y += 1;
                other.velocity.y -= 1;
            },
            Ordering::Equal => (),
        }
        match (self.position.z - other.position.z).cmp(&0) {
            Ordering::Greater => {
                self.velocity.z -= 1;
                other.velocity.z += 1;
            },
            Ordering::Less => {
                self.velocity.z += 1;
                other.velocity.z -= 1;
            },
            Ordering::Equal => (),
        }
    }

    fn energy(&self) -> usize {
        let p = self.position.x.abs() as usize + self.position.y.abs() as usize + self.position.z.abs() as usize;
        let k = self.velocity.x.abs() as usize + self.velocity.y.abs() as usize + self.velocity.z.abs() as usize;
        p * k
    }
}

fn simulate(bodies: &mut Vec<Body>) {
    for i in 0..bodies.len() {
        let mut body1 = bodies[i].clone();
        for body2 in bodies.iter_mut().skip(i+1) {
            body1.interact(body2);
        }
        bodies[i] = body1;
    }
    for body in bodies.iter_mut() {
        body.update();
    }
}

impl AoCDay for Day12 {
    fn day(&self) -> usize {
        12
    }
    fn expected(&self) -> (Option<&'static str>, Option<&'static str>) {
        (None, None)
    }
    fn part1(&self, input: &str) -> Result<String, ErrorWrapper> {
        let mut bodies = BODIES.clone();
        for _t in 0..1000 {
            simulate(&mut bodies);
        }
        Ok(format!("{:?}", bodies.iter().map(|b| b.energy()).sum::<usize>()))
    }
    fn part2(&self, input: &str) -> Result<String, ErrorWrapper> {
        let mut bodies = BODIES.clone();
        macro_rules! dimension {
            ($n:tt) => {{
                let mut v = vec![];
                for body in bodies.iter() {
                    v.push(body.position.$n);
                    v.push(body.velocity.$n);
                }
                v
            }}
        }
        let mut i: usize = 0;
        let initial = (dimension!(x), dimension!(y), dimension!(z));
        let mut c: (usize, usize, usize) = (0, 0, 0);
        loop {
            simulate(&mut bodies);
            i += 1;
            if c.0 == 0 && initial.0 == dimension!(x) {
                c.0 = i;
            }
            if c.1 == 0 && initial.1 == dimension!(y) {
                c.1 = i;
            }
            if c.2 == 0 && initial.2 == dimension!(z) {
                c.2 = i;
            }
            if c.0 != 0 && c.1 != 0 && c.2 != 0 {
                break;
            }
        }
        Ok(format!("{}", math::lcm(c.0, math::lcm(c.1, c.2))))
    }
}

pub fn get_day() -> Box<dyn AoCDay> {
    Box::new(Day12)
}
