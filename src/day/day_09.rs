use crate::AoCDay;

pub struct Day09;

use crate::intcode::{parse_intcode, IntCodeMachine};

const INPUT: &'static str = include_str!("../input/day_09.txt");

impl AoCDay for Day09 {
    fn day(&self) -> i32 {
        09
    }
    fn part1(&self) -> String {
        let code = parse_intcode(INPUT).unwrap();
        let mut machine = IntCodeMachine::new(code, vec![1], 200);
        match machine.execute() {
            Ok(_) => format!("{}", machine.output_buffer.pop().unwrap()),
            Err(e) => format!("Machine crashed! {:?}", e),
        }
    }
    fn part2(&self) -> String {
        let code = parse_intcode(INPUT).unwrap();
        let mut machine = IntCodeMachine::new(code, vec![2], 2000);
        match machine.execute() {
            Ok(_) => format!("{}", machine.output_buffer.pop().unwrap()),
            Err(e) => format!("Machine crashed! {:?}", e),
        }
    }
}

pub fn get_day() -> Box<dyn AoCDay> {
    Box::new(Day09)
}
