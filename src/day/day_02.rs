use aoc_core::{AoCDay, ErrorWrapper};

use intcode::{parse_intcode, IntCodeMachine, Num};

struct Day02;

impl AoCDay for Day02 {
    fn day(&self) -> usize {
        2
    }

    fn expected(&self) -> (Option<&'static str>, Option<&'static str>) {
        (Some("4090689"), Some("77, 33"))
    }
    fn part1(&self, input: &str) -> Result<String, ErrorWrapper> {
        let mut intcode: Vec<Num> = parse_intcode(input).expect("Invalid intcode");
        intcode[1] = 12;
        intcode[2] = 2;
        let mut machine = IntCodeMachine::new(intcode, vec![], 100);
        machine.execute()
            .map_err(|m| format!("Error executing program: {:?}", m))?;
        Ok(format!("{}", machine.memory.read_raw(0).unwrap()))
    }
    fn part2(&self, input: &str) -> Result<String, ErrorWrapper> {
        let intcode: Vec<Num> = parse_intcode(input).expect("Invalid intcode");
        let max = 99;
        let mut first = 0;
        let mut second = 0;
        loop {
            let mut i = intcode.clone();
            i[1] = first;
            i[2] = second;
            let mut machine = IntCodeMachine::new(i, vec![], 100);
            let r = machine.execute();
            if r.is_ok() && machine.memory.read_raw(0).unwrap() == 19690720 {
                break Ok(format!("{}, {}", first, second));
            } else {
                if second >= max {
                    first += 1;
                    second = 0;
                } else {
                    second += 1;
                }
            }
        }
    }
}

pub fn get_day() -> Box<dyn AoCDay> {
    Box::new(Day02)
}
