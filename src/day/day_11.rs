use aoc_core::{AoCDay, ErrorWrapper};
use intcode::{parse_intcode, IntCodeMachine, Num, ExecutionStatus};
use lazy_static::lazy_static;
use std::collections::HashMap;

type Point = aoc_core::Vec2<i32>;

pub struct Day11;

const INPUT: &str = include_str!("../input/day_11.txt");

lazy_static! {
    static ref INTCODE: Vec<Num> = parse_intcode(INPUT).expect("Input is not a valid intcode program");
}

#[derive(Debug, PartialEq)]
enum Paint {
    White,
    Black,
}
impl Paint {
    fn from(val: Num) -> Paint {
        match val {
            0 => Paint::Black,
            1 => Paint::White,
            _ => panic!("Bad paint!"),
        }
    }
    fn to_num(&self) -> Num {
        match self {
            Paint::Black => 0,
            Paint::White => 1,
        }
    }
}
#[derive(Debug, PartialEq)]
enum Facing {
    Up,
    Down,
    Left,
    Right,
}
impl Facing {
    fn turn(&self, direction: Num) -> Facing {
        match self {
            Facing::Up => match direction {
                0 => Facing::Left,
                1 => Facing::Right,
                _ => panic!("Bad turn!"),
            },
            Facing::Down => match direction {
                0 => Facing::Right,
                1 => Facing::Left,
                _ => panic!("Bad turn!"),
            },
            Facing::Left => match direction {
                0 => Facing::Down,
                1 => Facing::Up,
                _ => panic!("Bad turn!"),
            },
            Facing::Right => match direction {
                0 => Facing::Up,
                1 => Facing::Down,
                _ => panic!("Bad turn!"),
            },
        }
    }
}

fn run_paint_machine(starting_paint: Paint) -> (HashMap<Point, Paint>, usize) {
        let mut painted = 0;
        let intcode = INTCODE.clone();
        let mut machine = IntCodeMachine::new(intcode, vec![starting_paint.to_num()], 100);
        let mut grid: HashMap<Point, Paint> = HashMap::new();
        let mut position = Point::new(0, 0);
        let mut facing = Facing::Up;
        loop {
            match machine.execute().expect("Machine crashed!") {
                ExecutionStatus::Halted => break,
                ExecutionStatus::Blocking => {
                    facing = facing.turn(machine.output_buffer.pop().expect("Expected a facing output"));
                    let entry = grid.entry(position).or_insert_with(|| {
                        painted += 1;
                        Paint::Black
                    });
                    *entry = Paint::from(machine.output_buffer.pop().expect("Expected a paint output"));
                    match facing {
                        Facing::Up => position.y += 1,
                        Facing::Down => position.y -= 1,
                        Facing::Left => position.x -= 1,
                        Facing::Right => position.x += 1,
                    };
                    machine.input_buffer.push(grid.get(&position).unwrap_or(&Paint::Black).to_num());
                },
            }
        }
    (grid, painted)
}

impl AoCDay for Day11 {
    fn day(&self) -> usize {
        11
    }
    fn expected(&self) -> (Option<&'static str>, Option<&'static str>) {
        (None, None)
    }
    fn part1(&self) -> Result<String, ErrorWrapper> {
        Ok(format!("{}", run_paint_machine(Paint::Black).1))
    }
    fn part2(&self) -> Result<String, ErrorWrapper> {
        let (grid, _) = run_paint_machine(Paint::White);
        let max_y: i32 = grid.keys().max_by_key(|p| p.y).expect("Unable to get value").y;
        let min_y: i32 = grid.keys().min_by_key(|p| p.y).expect("Unable to get value").y;
        let min_x: i32 = grid.keys().min_by_key(|p| p.x).expect("Unable to get value").x;
        let max_x: i32 = grid.keys().max_by_key(|p| p.x).expect("Unable to get value").x;
        let mut cached_point = Point::new(0, 0);
        let block_char = crate::block_char();
        for y in (min_y..=max_y).rev() {
            let mut line = vec![];
            for x in min_x..max_x {
                cached_point.x = x;
                cached_point.y = y;
                line.push(match grid.get(&cached_point) {
                    None => ' ',
                    Some(p) => match p {
                        Paint::White => block_char,
                        Paint::Black => ' ',
                    }
                });
            }
            println!("{}", line.iter().collect::<String>());
        }

        Ok("Image written to console".to_string())
    }
}

pub fn get_day() -> Box<dyn AoCDay> {
    Box::new(Day11)
}
