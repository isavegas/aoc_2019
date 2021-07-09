// TODO: Finish day 15 and remove these allows
#![allow(unused_imports, dead_code)]

use aoc_core::{bail, AoCDay, ErrorWrapper, Vec2};
use intcode::{IntCodeMachine, parse_intcode, ExecutionStatus, Num };
use std::collections::HashMap;
use lazy_static::lazy_static;

type Point = Vec2<Num>;

pub struct Day15;

const INPUT: &str = include_str!("../input/day_15.txt");

lazy_static! {
    static ref INTCODE: Vec<Num> = parse_intcode(INPUT).expect("Unable to parse bundled intcode");
}

#[derive(Clone, Debug)]
pub enum Direction {
    North,
    South,
    East,
    West,
}

impl Direction {
    fn to_num(&self) -> Num {
        match self {
            Direction::North => 1,
            Direction::South => 2,
            Direction::West => 3,
            Direction::East => 4,
        }
    }
}

impl Direction {
    fn translate(&self, point: &mut Point) {
        match self {
            Direction::North => point.y += 1,
            Direction::South => point.y -= 1,
            Direction::East => point.x += 1,
            Direction::West => point.x -= 1,
        }
    }
    fn rotate(&self) -> Direction {
        match self {
            Direction::North => Direction::East,
            Direction::East => Direction::South,
            Direction::South => Direction::West,
            Direction::West => Direction::North,
        }
    }
    fn rotate_rev(&self) -> Direction {
        match self {
            Direction::North => Direction::West,
            Direction::East => Direction::North,
            Direction::South => Direction::East,
            Direction::West => Direction::South,
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
enum MoveResult {
    Failure,
    Success(bool),
    Invalid(Num),
}

impl From<Num> for MoveResult {
    fn from(n: Num) -> MoveResult {
        match n {
            0 => MoveResult::Failure,
            1 => MoveResult::Success(false),
            2 => MoveResult::Success(true),
            n => MoveResult::Invalid(n),
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum Tile {
    Wall,
    Empty,
    Oxygen,
    Origin,
    Unknown,
}

impl Default for Tile {
    fn default() -> Tile {
        Tile::Unknown
    }
}

pub fn render(map: &HashMap<Point, Tile>, center: &Point, direction: &Direction) {
    const WIDTH: usize = 100;
    const HEIGHT: usize = 100;
    let mut p = Point::default();
    let mut l = [' '; WIDTH];
    let ox = center.x - (WIDTH / 2) as Num;
    let oy = center.y - (HEIGHT / 2) as Num;

    for y in (0..HEIGHT).rev() {
        p.y = oy + y as Num;
        for (x, v) in l.iter_mut().enumerate() {
            p.x = ox + x as Num;
            *v = if &p == center {
                match direction {
                    Direction::North => '^',
                    Direction::South => 'v',
                    Direction::East => '>',
                    Direction::West => '<',
                }
            } else {
                match map.get(&p) {
                    Some(t) => match t {
                        Tile::Oxygen => 'O',
                        Tile::Wall => 'X',
                        Tile::Empty => ' ',
                        Tile::Origin => '@',
                        Tile::Unknown => '#',
                    },
                    None => '#',
                }
            }
        }
        //println!("{}", l.iter().collect::<String>());
    }
}

impl AoCDay for Day15 {
    fn day(&self) -> usize {
        15
    }
    fn expected(&self) -> (Option<&'static str>, Option<&'static str>) {
        (None, None)
    }
    fn part1(&self, input: &str) -> Result<String, ErrorWrapper> {
/*        let mut map: HashMap<Point, Tile> = HashMap::new();
        let mut position = Point::default();
        let mut direction = Direction::North;
        map.insert(position.clone(), Tile::Origin);

        let mut droid = IntCodeMachine::new(INTCODE.clone(), vec![direction.to_num()], 1000);
        let mut _queue: Vec<Point> = Vec::with_capacity(256);
        match loop {
            match droid.execute() {
                Ok(status) => match status {
                    ExecutionStatus::Halted => {
                        if let MoveResult::Success(oxygen) = MoveResult::from(droid.output_buffer.pop().expect("Invalid output")) {
                            if oxygen {
                                break Ok(());
                            }
                        }
                        break Err(());
                    },
                    ExecutionStatus::Blocking => {
                        match droid.output_buffer.pop().expect("No output").into() {
                            MoveResult::Success(found) => {
                                // Update position and map
                                direction.translate(&mut position);
                                direction = direction.rotate_rev();
                                if found {
                                    map.insert(position.clone(), Tile::Oxygen);
                                    // break Ok(());
                                } else {
                                    let _ = map.entry(position.clone()).or_insert(Tile::Empty);
                                }
                            },
                            MoveResult::Failure => {
                                let mut wall_pos = position.clone();
                                direction.translate(&mut wall_pos);
                                map.insert(wall_pos, Tile::Wall);
                                direction = direction.rotate();
                            },
                            _ => break Err(()),
                        }
                        //println!();
                        render(&map, &position, &direction);
                        droid.input_buffer.push(direction.to_num());
                    },
                },
                Err(_err) => break Err(()),
            }
        } {
            Ok(_) => format!("{}", position),
            Err(_) => format!("Droid crashed!"),
        }*/
        unimplemented!()
    }
    fn part2(&self, input: &str) -> Result<String, ErrorWrapper> {
        unimplemented!()
    }
}

pub fn get_day() -> Box<dyn AoCDay> {
    Box::new(Day15)
}
