use crate::AoCDay;
use crate::Vec2;
use crate::intcode::{ IntCodeMachine, parse_intcode, ExecutionStatus, Num };
use std::collections::HashMap;
use lazy_static::lazy_static;

type Point = Vec2<i32>;

pub struct Day15;

const INPUT: &'static str = include_str!("../input/day_15.txt");

lazy_static! {
    static ref INTCODE: Vec<Num> = parse_intcode(INPUT).expect("Unable to parse bundled intcode");
}

#[derive(Clone, Debug)]
enum Direction {
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
}

#[derive(Clone, Debug, PartialEq)]
enum MoveResult {
    Failure,
    Success(bool),
}

impl From<Num> for MoveResult {
    fn from(n: Num) -> MoveResult {
        match n {
            0 => MoveResult::Failure,
            1 => MoveResult::Success(false),
            2 => MoveResult::Success(true),
            _ => unimplemented!("TODO: From<Num> for MoveResult"),
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
enum Tile {
    Wall,
    Empty,
    Oxygen,
    Unknown,
}

impl Default for Tile {
    fn default() -> Tile {
        Tile::Unknown
    }
}

#[derive(Clone, Debug, PartialEq, Default)]
struct Chunk {
    data: [[Tile; 32]; 32]
}

#[derive(Clone, Debug)]
struct Map<'a> {
    pub size: Point,
    pub current: &'a Chunk,
    pub chunks: HashMap<Point, Chunk>,
}
impl<'a> Map<'_> {
    #[allow(dead_code)]
    fn new() -> Map<'a> {
/*
        let mut chunks = HashMap::new();
        Map {
            size: Point::new(32, 32),
            current: chunks.entry(Point::default()).or_default(),
            chunks,
        }
*/
        unimplemented!()
    }
    /*fn chunk(&mut self, p: &Point) {
        self.current = self.chunks.entry(p % &self.size).or_default();
    }*/
}

impl AoCDay for Day15 {
    fn day(&self) -> usize {
        15
    }
    fn part1(&self) -> String {
        let mut map: HashMap<Point, Tile> = HashMap::new();
        let mut position = Point::default();
        let mut direction = Direction::North;
        map.insert(position.clone(), Tile::Empty);

        let mut droid = IntCodeMachine::new(INTCODE.clone(), vec![direction.to_num()], 1000);
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
                                if found {
                                    map.insert(position.clone(), Tile::Oxygen);
                                    break Ok(());
                                } else {
                                    map.insert(position.clone(), Tile::Empty);
                                }
                            },
                            MoveResult::Failure => {
                                let mut wall_pos = position.clone();
                                direction.translate(&mut wall_pos);
                                if map.get(&wall_pos).is_some() {
                                    direction = direction.rotate();
                                    direction = direction.rotate();
                                }
                                map.insert(wall_pos, Tile::Wall);
                                direction = direction.rotate();
                            },
                        }
                        println!("{:?}", map);
                        droid.input_buffer.push(Direction::North.to_num());
                    },
                },
                Err(_err) => break Err(()),
            }
        } {
            Ok(_) => format!("{}", position),
            Err(_) => format!("Droid crashed!"),
        }
    }
    fn part2(&self) -> String {
        unimplemented!()
    }
}

pub fn get_day() -> Box<dyn AoCDay> {
    Box::new(Day15)
}
