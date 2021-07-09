pub use aoc_core::{bail, ensure, AoCDay, ErrorWrapper};
use std::collections::HashMap;
use std::str::FromStr;

pub struct Day03;

const INPUT: &str = include_str!("../input/day_03.txt");

type Num = isize;

#[derive(Debug, PartialEq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug, PartialEq)]
struct PathComponent {
    direction: Direction,
    distance: Num,
}

impl PathComponent {
    fn partial_apply(&self, p: &Point, d: Num) -> Point {
        match &self.direction {
            Direction::Up => Point { x: p.x, y: p.y + d },
            Direction::Down => Point { x: p.x, y: p.y - d },
            Direction::Left => Point { x: p.x - d, y: p.y },
            Direction::Right => Point { x: p.x + d, y: p.y },
        }
    }
}

impl FromStr for PathComponent {
    type Err = aoc_core::ErrorWrapper;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() >= 2 {
            let direction = match s.chars().next().unwrap() {
                'U' => Direction::Up,
                'D' => Direction::Down,
                'L' => Direction::Left,
                'R' => Direction::Right,
                _ => bail!("Invalid Direction for PathComponent: {}", s),
            };
            let distance = s[1..].parse::<Num>()?;
            Ok(PathComponent {
                direction,
                distance,
            })
        } else {
            bail!("Incorrect number of parts in PathComponent: {}", s)
        }
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
struct Point {
    x: Num,
    y: Num,
}

impl Point {
    fn manhattan_distance(&self, o: &Point) -> Num {
        manhattan_distance(self, o)
    }
}

fn manhattan_distance(p1: &Point, p2: &Point) -> Num {
    (p1.x - p2.x).abs() + (p1.y - p2.y).abs()
}

impl AoCDay for Day03 {
    fn day(&self) -> usize {
        3
    }
    fn expected(&self) -> (Option<&'static str>, Option<&'static str>) {
        (None, None)
    }
    fn part1(&self) -> Result<String, ErrorWrapper> {
        let origin = Point { x: 0, y: 0 };
        let input = get_input();
        let mut visited: HashMap<Point, bool> = HashMap::new();
        let mut collisions = vec![];
        let mut last = origin.clone();
        for pc in input[0].iter() {
            for _ in 0..pc.distance {
                last = pc.partial_apply(&last, 1);
                visited.insert(last.clone(), true);
            }
        }
        last = origin.clone();
        for pc in input[1].iter() {
            for _ in 0..pc.distance {
                last = pc.partial_apply(&last, 1);
                if visited.get(&last).is_some() {
                    collisions.push(last.clone());
                }
            }
        }
        Ok(format!(
            "{}",
            collisions
                .iter()
                .map(|p| p.manhattan_distance(&origin))
                .min()
                .unwrap()
        ))
    }
    fn part2(&self) -> Result<String, ErrorWrapper> {
        let origin = Point { x: 0, y: 0 };
        let input = get_input();
        let mut visited: HashMap<Point, usize> = HashMap::new();
        let mut collisions = vec![];
        let mut last = origin.clone();
        let mut n = 0;
        for pc in input[0].iter() {
            for _ in 0..pc.distance {
                last = pc.partial_apply(&last, 1);
                n += 1;
                if !visited.contains_key(&last) {
                    visited.insert(last.clone(), n);
                }
            }
        }
        last = origin;
        n = 0;
        for pc in input[1].iter() {
            for _ in 0..pc.distance {
                n += 1;
                last = pc.partial_apply(&last, 1);
                if let Some(v) = visited.get(&last) {
                    collisions.push(v + n);
                }
            }
        }
        Ok(format!("{}", collisions.iter().min().unwrap()))
    }
}

pub fn get_day() -> Box<dyn AoCDay> {
    Box::new(Day03)
}

fn get_input() -> Vec<Vec<PathComponent>> {
    INPUT
        .trim()
        .split('\n')
        .map(String::from)
        .map(|s| {
            s.split(',')
                .map(|p| p.trim().parse().expect("INVALID INPUT FOR DAY 3"))
                .collect::<Vec<PathComponent>>()
        })
        .collect::<Vec<Vec<PathComponent>>>()
}
