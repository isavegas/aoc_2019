use crate::{AoCDay, ErrorWrapper};
use lazy_static::lazy_static;

pub struct Day02;

const INPUT: &str = include_str!("../input/day_02.txt");

use intcode::{parse_intcode, IntCodeMachine, Num};

lazy_static! {
    // This should ALWAYS succeed.
    static ref INTCODE: Vec<Num> = parse_intcode(INPUT).expect("Invalid intcode bundled into application");
}

impl AoCDay for Day02 {
    fn day(&self) -> usize {
        2
    }

    fn expected(&self) -> (Option<&'static str>, Option<&'static str>) {
        (Some("4090689"), Some("77, 33"))
    }
    fn part1(&self) -> Result<String, ErrorWrapper> {
        let mut input = INTCODE.clone();
        input[1] = 12;
        input[2] = 2;
        let mut machine = IntCodeMachine::new(input, vec![], 100);
        let err = machine.execute();
        if err.is_err() {
            println!("Error running machine! {:?}", err);
        }
        Ok(format!("{}", machine.memory.read_raw(0).unwrap()))
    }
    fn part2(&self) -> Result<String, ErrorWrapper> {
        let input = INTCODE.clone();
        let max = 99;
        let mut first = 0;
        let mut second = 0;
        loop {
            let mut i = input.clone();
            i[1] = first;
            i[2] = second;
            let mut machine = IntCodeMachine::new(i, vec![], 100);
            let r = machine.execute();
            if r.is_ok() && machine.memory.read_raw(0).unwrap() == 19690720 {
                break Ok(format!("{}, {}", first, second));
            } else {
                //println!("{}, {} => {}", first, second, machine.memory[0]);
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
