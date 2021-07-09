use intcode::{parse_intcode, ExecutionStatus, IntCodeMachine, Num};
use crate::{AoCDay, ErrorWrapper};
use lazy_static::lazy_static;
use permutohedron::Heap;

pub struct Day07;

const INPUT: &str = include_str!("../input/day_07.txt");

lazy_static! {
    // This should ALWAYS succeed.
    static ref INTCODE: Vec<Num> = parse_intcode(INPUT).expect("Unable to parse input");
}

impl AoCDay for Day07 {
    fn day(&self) -> usize {
        7
    }
    fn expected(&self) -> (Option<&'static str>, Option<&'static str>) {
        (None, None)
    }
    fn part1(&self) -> Result<String, ErrorWrapper> {
        Ok(Heap::new(&mut [0, 1, 2, 3, 4])
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
            .ok_or_else(|| ErrorWrapper::Simple("No max found".to_string()))?
            .to_string())
    }
    fn part2(&self) -> Result<String, ErrorWrapper> {
        fn new_machine(phase: Num) -> IntCodeMachine {
            IntCodeMachine::new(INTCODE.clone(), vec![phase], 100)
        }
        Ok(Heap::new(&mut [5, 6, 7, 8, 9])
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
            .ok_or_else(|| ErrorWrapper::Simple("No max found".to_string()))?
            .to_string())
    }
}

pub fn get_day() -> Box<dyn AoCDay> {
    Box::new(Day07)
}
