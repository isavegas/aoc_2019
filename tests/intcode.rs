use aoc_2019::intcode::{parse_intcode, IntCodeMachine, Num, ExecutionStatus, ErrorStatus};

fn execute(intcode: &str) -> (IntCodeMachine, Result<ExecutionStatus, ErrorStatus>) {
    run_code(parse_intcode(intcode).unwrap(), vec![])
}

fn run_code(intcode: Vec<Num>, input: Vec<Num>) -> (IntCodeMachine, Result<ExecutionStatus, ErrorStatus>) {
    let mut machine = IntCodeMachine::new(intcode, input);
    let r = machine.execute();
    (machine, r)
}

#[test]
fn add_position() {
    assert_eq!(execute("1,5,5,0,99,3").0.memory[0], 6, "Fails to add positional parameters")
}

#[test]
fn add_immediate() {
    assert_eq!(execute("1101,5,5,0,99,3").0.memory[0], 10, "Fails to add immediate parameters")
}

#[test]
fn add_mixed() {
    assert_eq!(execute("1001,5,5,0,99,3").0.memory[0], 8, "Fails to add mixed mode parameters")
}

#[test]
fn add_immediate_output_param() {
    assert_eq!(execute("10001,5,5,0,99,3").0.memory[0], 6, "Fails to ignore mode for add output parameter")
}

#[test]
fn mul_position() {
    assert_eq!(execute("2,5,5,0,99,3").0.memory[0], 9, "Fails to multiply positional parameters")
}

#[test]
fn mul_immediate() {
    assert_eq!(execute("1102,5,5,0,99,3").0.memory[0], 25, "Fails to multiply immediate parameters")
}

#[test]
fn mul_mixed() {
    assert_eq!(execute("1002,5,5,0,99,3").0.memory[0], 15, "Fails to multiply mixed mode parameters")
}

#[test]
fn mul_immediate_output_param() {
    assert_eq!(execute("10002,5,5,0,99,3").0.memory[0], 9, "Fails to ignore mode for mul output parameter")
}

#[test]
fn input_nonblocking() {
    assert_eq!(run_code(parse_intcode("3,0,99").unwrap(), vec![4]).0.memory[0], 4, "Fails to get existing input data without blocking")
}
