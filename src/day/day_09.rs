use crate::AoCDay;

pub struct Day09;

use crate::intcode::{parse_intcode, IntCodeMachine};

const INPUT: &'static str = include_str!("../input/day_09.txt");

impl AoCDay for Day09 {
    fn day(&self) -> i32 {
        09
    }
    fn part1(&self) -> String {
        let now = std::time::Instant::now();
        let mut code = parse_intcode(INPUT).unwrap();
        let mut machine = IntCodeMachine::new(code, vec![1], 2000);
        machine.log_ops = true;
        if let Err(e) = machine.execute() {
            return format!("Machine crashed! {:?}", e);
        } else {
            println!("{:?}", now.elapsed().as_nanos());
            format!("{:?}", machine.output_buffer)
        }
    }
    fn part2(&self) -> String {
        let mut code = parse_intcode(INPUT).unwrap();
            code.push(0);
        let mut machine = IntCodeMachine::new(code, vec![2], 100);
        if let Ok(s) = machine.execute() {
            println!("{:?}", s);
        }
        format!("{:?}", machine.output_buffer)
    }
}

pub fn get_day() -> Box<dyn AoCDay> {
    Box::new(Day09)
}
