use crate::AoCDay;

pub struct Day09;

use crate::intcode::{parse_intcode, Num, IntCodeMachine};

//const INPUT: &'static str = include_str!("../input/day_09.txt");
const INPUT: &'static str = "109,1,204,-1,1001,100,1,100,1008,100,16,101,1006,101,0,99";

impl AoCDay for Day09 {
    fn day(&self) -> i32 {
        09
    }
    fn part1(&self) -> String {
        let mut code = parse_intcode(INPUT).unwrap();
        for i in 0..10000 {
            code.push(0);
        }
        let mut machine = IntCodeMachine::new(code, vec![1]);
        if let Ok(s) = machine.execute() {
            println!("{:?}", s);
        }
        format!("{:?}", machine.output_buffer)
    }
    fn part2(&self) -> String {
        let mut code = parse_intcode(INPUT).unwrap();
        for i in 0..10000 {
            code.push(0);
        }
        let mut machine = IntCodeMachine::new(code, vec![2]);
        if let Ok(s) = machine.execute() {
            println!("{:?}", s);
        }
        format!("{:?}", machine.output_buffer)
    }
}

pub fn get_day() -> Box<dyn AoCDay> {
    Box::new(Day09)
}
