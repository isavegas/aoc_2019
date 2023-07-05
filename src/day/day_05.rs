use aoc_core::{AoCDay, ErrorWrapper};

pub struct Day05;

use intcode::{parse_intcode, IntCodeMachine, Num};

impl AoCDay for Day05 {
    fn day(&self) -> usize {
        5
    }
    fn expected(&self) -> (Option<&'static str>, Option<&'static str>) {
        (None, None)
    }
    fn part1(&self, input: &str) -> Result<String, ErrorWrapper> {
        let intcode = parse_intcode(input).expect("Invalid intcode");
        let mut machine = IntCodeMachine::new(intcode.clone(), vec![1], 100);
        let err = machine.execute();
        if err.is_err() {
            println!("Error running machine! {:?}", err);
        }
        Ok(format!(
            "{:?}",
            machine.output_buffer[machine.output_buffer.len() - 1]
        ))
    }
    fn part2(&self, input: &str) -> Result<String, ErrorWrapper> {
        let intcode = parse_intcode(input).expect("Invalid intcode");
        let mut machine = IntCodeMachine::new(intcode.clone(), vec![5], 100);
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
