use aoc_core::{AoCDay, ErrorWrapper};
use intcode::{parse_intcode, ExecutionStatus, IntCodeMachine, Num};
use permutohedron::Heap;

pub struct Day07;

impl AoCDay for Day07 {
    fn day(&self) -> usize {
        7
    }
    fn expected(&self) -> (Option<&'static str>, Option<&'static str>) {
        (None, None)
    }
    fn part1(&self, input: &str) -> Result<String, ErrorWrapper> {
        let intcode = parse_intcode(input).expect("Invalid intcode");
        Ok(Heap::new(&mut [0, 1, 2, 3, 4])
            .map(|params| {
                let mut signal = 0;
                for p in &params {
                    let mut machine = IntCodeMachine::new(intcode.clone(), vec![*p, signal], 100);
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
    fn part2(&self, input: &str) -> Result<String, ErrorWrapper> {
        let intcode = parse_intcode(input).expect("Invalid intcode");
        fn new_machine(phase: Num, code: Vec<isize>) -> IntCodeMachine {
            IntCodeMachine::new(code, vec![phase], 100)
        }
        Ok(Heap::new(&mut [5, 6, 7, 8, 9])
            .map(|params| {
                let mut machines: Vec<_> = params
                    .iter()
                    .map(|p| new_machine(*p, intcode.clone()))
                    .collect();
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
