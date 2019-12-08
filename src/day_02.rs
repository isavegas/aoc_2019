use crate::AoCDay;
use lazy_static::lazy_static;

pub struct Day02;

const INPUT: &'static str = include_str!("./input/day_02.txt");

use crate::intcode::{parse_intcode, IntCodeMachine, Num};

lazy_static! {
    // This should ALWAYS succeed.
    static ref INTCODE: Vec<Num> = parse_intcode(INPUT).expect("Invalid intcode bundled into application");
}

impl AoCDay for Day02 {
    fn day(&self) -> i32 {
        2
    }
    fn part1(&self) -> String {
        let mut input = INTCODE.clone();
        input[1] = 12;
        input[2] = 2;
        let mut machine = IntCodeMachine::new(input, vec![]);
        let err = machine.execute();
        if err.is_err() {
            println!("Error running machine! {:?}", err);
        }
        format!("{}", machine.memory[0])
    }
    fn part2(&self) -> String {
        let input = INTCODE.clone();
        let max = 99;
        let mut first = 0;
        let mut second = 0;
        loop {
            let mut i = input.clone();
            i[1] = first;
            i[2] = second;
            let mut machine = IntCodeMachine::new(i, vec![]);
            let r = machine.execute();
            if r.is_ok() && machine.memory[0] == 19690720 {
                break format!("{}, {}", first, second);
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
