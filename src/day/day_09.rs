use aoc_core::{AoCDay, ErrorWrapper};

pub struct Day09;

use intcode::{parse_intcode, IntCodeMachine};

const INPUT: &str = include_str!("../input/day_09.txt");

impl AoCDay for Day09 {
    fn day(&self) -> usize {
        9
    }
    fn expected(&self) -> (Option<&'static str>, Option<&'static str>) {
        (None, None)
    }
    fn part1(&self) -> Result<String, ErrorWrapper> {
        let code = parse_intcode(INPUT).unwrap();
        let mut machine = IntCodeMachine::new(code, vec![1], 200);
        machine.execute()
            .map(|_| format!("{}", machine.output_buffer.pop().unwrap()))
            .map_err(|e| ErrorWrapper::Simple(format!("{:?}", e)))
        /* match machine.execute() {
            Ok(_) => format!("{}", machine.output_buffer.pop().unwrap()),
            Err(e) => format!("Machine crashed! {:?}", e),
        } */
    }
    fn part2(&self) -> Result<String, ErrorWrapper> {
        let code = parse_intcode(INPUT).unwrap();
        let mut machine = IntCodeMachine::new(code, vec![2], 2000);
        machine.execute()
            .map(|_| format!("{}", machine.output_buffer.pop().unwrap()))
            .map_err(|e| ErrorWrapper::Simple(format!("{:?}", e)))
        /* match machine.execute() {
            Ok(_) => format!("{}", machine.output_buffer.pop().unwrap()),
            Err(e) => format!("Machine crashed! {:?}", e),
        } */
    }
}

pub fn get_day() -> Box<dyn AoCDay> {
    Box::new(Day09)
}
