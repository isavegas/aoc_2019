use crate::intcode::{parse_intcode, ExecutionStatus, IntCodeMachine, Num};
use crate::AoCDay;
use lazy_static::lazy_static;
use permutohedron::Heap;

pub struct Day07;

const INPUT: &'static str = include_str!("../input/day_07.txt");

lazy_static! {
    // This should ALWAYS succeed.
    static ref INTCODE: Vec<Num> = parse_intcode(INPUT).expect("Unable to parse input");
}

impl AoCDay for Day07 {
    fn day(&self) -> usize {
        07
    }
    fn part1(&self) -> String {
        Heap::new(&mut [0, 1, 2, 3, 4])
            .map(|params| {
                let mut signal = 0;
                for p in &params {
                    let mut machine = IntCodeMachine::new(INTCODE.clone(), vec![*p, signal], 100);
                    assert!(
                        machine.execute().expect("Machine crashed!") == ExecutionStatus::Halted,
                        "Machine blocking!"
                    );
                    signal = machine
                        .output_buffer
                        .pop()
                        .expect("No output from machine!");
                }
                signal
            })
            .max()
            .unwrap()
            .to_string()
    }
    fn part2(&self) -> String {
        fn new_machine(phase: Num) -> IntCodeMachine {
            IntCodeMachine::new(INTCODE.clone(), vec![phase], 100)
        }
        Heap::new(&mut [5, 6, 7, 8, 9])
            .map(|params| {
                let mut machines: Vec<_> = params.iter().map(|p| new_machine(*p)).collect();
                let mut signal = 0;
                let mut i = 0;
                while !machines[4].halt {
                    machines[i].input_buffer.push(signal);
                    machines[i].execute().expect("Machine crashed!");
                    signal = machines[i]
                        .output_buffer
                        .pop()
                        .expect("No output from machine!");

                    if i == 4 {
                        i = 0;
                    } else {
                        i += 1;
                    }
                }
                signal
            })
            .max()
            .unwrap()
            .to_string()
    }
}

pub fn get_day() -> Box<dyn AoCDay> {
    Box::new(Day07)
}
