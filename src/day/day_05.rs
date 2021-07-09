use aoc_core::{AoCDay, ErrorWrapper};
use lazy_static::lazy_static;

pub struct Day05;

const INPUT: &str = include_str!("../input/day_05.txt");

use intcode::{parse_intcode, IntCodeMachine, Num};

lazy_static! {
    // This should ALWAYS succeed.
    static ref INTCODE: Vec<Num> = parse_intcode(INPUT).expect("Invalid intcode bundled into application");
}

impl AoCDay for Day05 {
    fn day(&self) -> usize {
        5
    }
    fn expected(&self) -> (Option<&'static str>, Option<&'static str>) {
        (None, None)
    }
    fn part1(&self) -> Result<String, ErrorWrapper> {
        let mut machine = IntCodeMachine::new(INTCODE.clone(), vec![1], 100);
        let err = machine.execute();
        if err.is_err() {
            println!("Error running machine! {:?}", err);
        }
        Ok(format!(
            "{:?}",
            machine.output_buffer[machine.output_buffer.len() - 1]
        ))
    }
    fn part2(&self) -> Result<String, ErrorWrapper> {
        let mut machine = IntCodeMachine::new(INTCODE.clone(), vec![5], 100);
        let err = machine.execute();
        if err.is_err() {
            println!("Error running machine! {:?}", err);
        }
        Ok(format!(
            "{:?}",
            machine.output_buffer[machine.output_buffer.len() - 1]
        ))
    }
}

pub fn get_day() -> Box<dyn AoCDay> {
    Box::new(Day05)
}
