mod instructions {
    use aoc_2019::intcode::{parse_intcode, ErrorStatus, ExecutionStatus, IntCodeMachine, Num};

    fn execute(intcode: &str) -> (IntCodeMachine, Result<ExecutionStatus, ErrorStatus>) {
        run_code(parse_intcode(intcode).unwrap(), vec![])
    }

    fn run_code(
        intcode: Vec<Num>,
        input: Vec<Num>,
    ) -> (IntCodeMachine, Result<ExecutionStatus, ErrorStatus>) {
        let mut machine = IntCodeMachine::new(intcode, input, 100);
        let r = machine.execute();
        (machine, r)
    }

    #[test]
    fn add_position() {
        let (machine, result) = execute("1,5,5,0,99,3");
        assert!(
            result.is_ok(),
            format!("Crashed! {:?}", result.unwrap_err())
        );
        assert_eq!(
            machine.memory.read_raw(0).unwrap(),
            6,
            "Fails to add positional parameters"
        )
    }

    #[test]
    fn add_immediate() {
        let (machine, result) = execute("1101,5,5,0,99,3");
        assert!(
            result.is_ok(),
            format!("Crashed! {:?}", result.unwrap_err())
        );
        assert_eq!(
            machine.memory.read_raw(0).unwrap(),
            10,
            "Fails to add immediate parameters"
        )
    }

    #[test]
    fn add_mixed() {
        let (machine, result) = execute("1001,5,5,0,99,3");
        assert!(
            result.is_ok(),
            format!("Crashed! {:?}", result.unwrap_err())
        );
        assert_eq!(
            machine.memory.read_raw(0).unwrap(),
            8,
            "Fails to add mixed mode parameters"
        )
    }

    #[test]
    fn add_immediate_output_param() {
        let (machine, result) = execute("10001,5,5,0,99,3");
        assert!(
            result.is_ok(),
            format!("Crashed! {:?}", result.unwrap_err())
        );
        assert_eq!(
            machine.memory.read_raw(0).unwrap(),
            6,
            "Fails to ignore mode for add output parameter"
        )
    }

    #[test]
    fn mul_position() {
        let (machine, result) = execute("2,5,5,0,99,3");
        assert!(
            result.is_ok(),
            format!("Crashed! {:?}", result.unwrap_err())
        );
        assert_eq!(
            machine.memory.read_raw(0).unwrap(),
            9,
            "Fails to multiply positional parameters"
        )
    }

    #[test]
    fn mul_immediate() {
        let (machine, result) = execute("1102,5,5,0,99,3");
        assert!(
            result.is_ok(),
            format!("Crashed! {:?}", result.unwrap_err())
        );
        assert_eq!(
            machine.memory.read_raw(0).unwrap(),
            25,
            "Fails to multiply immediate parameters"
        )
    }

    #[test]
    fn mul_mixed() {
        let (machine, result) = execute("1002,5,5,0,99,3");
        assert!(
            result.is_ok(),
            format!("Crashed! {:?}", result.unwrap_err())
        );
        assert_eq!(
            machine.memory.read_raw(0).unwrap(),
            15,
            "Fails to multiply mixed mode parameters"
        )
    }

    #[test]
    fn mul_immediate_output_param() {
        let (machine, result) = execute("10002,5,5,0,99,3");
        assert!(
            result.is_ok(),
            format!("Crashed! {:?}", result.unwrap_err())
        );
        assert_eq!(
            machine.memory.read_raw(0).unwrap(),
            9,
            "Fails to ignore mode for mul output parameter"
        )
    }

    #[test]
    fn input() {
        let (machine, result) = run_code(parse_intcode("3,0,99").unwrap(), vec![4]);
        assert!(
            result.is_ok(),
            format!("Crashed! {:?}", result.unwrap_err())
        );
        assert_eq!(
            machine.memory.read_raw(0).unwrap(),
            4,
            "Fails to get existing input data without blocking"
        )
    }

    #[test]
    fn input() {
        let (machine, result) = run_code(parse_intcode("3,0,99").unwrap(), vec![4]);
        assert!(
            result.is_ok(),
            format!("Crashed! {:?}", result.unwrap_err())
        );
        assert_eq!(
            machine.memory.read_raw(0).unwrap(),
            4,
            "Fails to get existing input data without blocking"
        )
    }
    #[test]
    fn output() {
        let (machine, result) = run_code(parse_intcode("4,0,99").unwrap(), vec![]);
        assert!(
            result.is_ok(),
            format!("Crashed! {:?}", result.unwrap_err())
        );
        assert_eq!(
            machine.output_buffer.get(0),
            Some(&4_isize),
            "Fails to get existing input data without blocking"
        );
    }
}

mod memory {
    use aoc_2019::intcode::{Memory, ParamMode};
    #[test]
    fn from_vec() {
        let mem = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11];
        let mut mem_out = mem.clone();
        mem_out.resize_with(12, || 0);
        assert_eq!(
            Memory::from_vec(2, mem.clone()).flatten(),
            mem_out,
            "Doesn't map a vector to pages correctly"
        );
    }
    #[test]
    fn read_raw() {
        let mem = Memory::from_vec(10, vec![1, 2, 3, 4, 5, 6, 7, 8, 9]);
        assert_eq!(mem.read_raw(0), Ok(1), "Fails to read from memory");
    }
    #[test]
    fn write_raw() {
        let mut mem = Memory::from_vec(10, vec![1, 2, 3, 4, 5, 6, 7, 8, 9]);
        mem.write_raw(0, 5).unwrap();
        assert_eq!(mem.flatten()[0], 5, "Fails to write to memory");
    }
    #[test]
    fn read_position() {
        let mem = Memory::from_vec(10, vec![1, 2, 3, 4, 5, 6, 7, 8, 9]);
        assert_eq!(
            mem.read(0, ParamMode::Position),
            Ok(2),
            "Fails to write to memory"
        );
    }
    #[test]
    fn read_relative() {
        let mut mem = Memory::from_vec(10, vec![1, 2, 3, 4, 5, 6, 7, 8, 9]);
        mem.relative_base = 1;
        assert_eq!(
            mem.read(1, ParamMode::Relative),
            Ok(4),
            "Fails to write to memory"
        );
    }
    #[test]
    fn write_position() {
        let mut mem = Memory::from_vec(10, vec![1, 2, 3, 4, 5, 6, 7, 8, 9]);
        mem.write(0, 5, ParamMode::Position).unwrap();
        assert_eq!(mem.read_raw(1), Ok(5), "Fails to write in positional mode");
    }
}
