pub type Num = isize;

#[derive(Debug, Clone, PartialEq)]
pub enum ErrorStatus {
    UnterminatedProgram,
    UnrecognizedOp(Num),
    OutOfBounds,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ParamMode {
    Immediate,
    Position,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Operand {
    val: Num,
    mode: ParamMode,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Op {
    Add,
    Mul,
    Input,
    Output,
    JNZ,
    JZ,
    LT,
    EQ,
    GT,
    JumpA,
    JumpR,
    Halt,
}

impl Op {
    pub fn new(n: Num) -> Result<Op, ErrorStatus> {
        match n {
            1 => Ok(Op::Add),
            2 => Ok(Op::Mul),
            3 => Ok(Op::Input),
            4 => Ok(Op::Output),
            /* These might change opcodes. */
            5 => Ok(Op::JNZ),
            6 => Ok(Op::JZ),
            7 => Ok(Op::LT),
            8 => Ok(Op::EQ),
            9 => Ok(Op::GT),
            10 => Ok(Op::JumpA),
            11 => Ok(Op::JumpR),
            99 => Ok(Op::Halt),
            _ => Err(ErrorStatus::UnrecognizedOp(n)),
        }
    }

    // returns the amount of opcodes consumed. Should always be at least 1
    pub fn apply(
        &self,
        modes: &[ParamMode],
        machine: &mut IntCodeMachine,
    ) -> Result<usize, ErrorStatus> {
        //fn get_param(n: usize) -> Result<Num, ErrorStatus> {
        let get_param = |offset: usize| match modes[offset - 1] {
            ParamMode::Immediate => machine
                .memory
                .get(machine.ip + offset)
                .ok_or(ErrorStatus::OutOfBounds)
                .map(|f| *f),
            ParamMode::Position => {
                let address = machine
                    .memory
                    .get(machine.ip + offset)
                    .ok_or(ErrorStatus::OutOfBounds)
                    .map(|f| *f)
                    .unwrap();
                machine
                    .memory
                    .get(address as usize)
                    .ok_or(ErrorStatus::OutOfBounds)
                    .map(|f| *f)
            }
        };

        let write_val = |m: &mut IntCodeMachine, pos: usize, val: Num| -> Result<(), ErrorStatus> {
            let target = *m.memory.get(pos).ok_or(ErrorStatus::OutOfBounds)? as usize;
            *m.memory.get_mut(target).ok_or(ErrorStatus::OutOfBounds)? = val;
            Ok(())
        };
        fn to_num(b: bool) -> Num {
            match b {
                true => 1,
                false => 0,
            }
        }
        match self {
            Op::Add => {
                let p1 = get_param(1)?;
                let p2 = get_param(2)?;
                write_val(machine, machine.ip + 3, p1 + p2)?;
            }
            Op::Mul => {
                let p1 = get_param(1)?;
                let p2 = get_param(2)?;
                write_val(machine, machine.ip + 3, p1 * p2)?;
            }
            Op::Input => {
                let n = machine.input_buffer[machine.input_pointer];
                write_val(machine, machine.ip + 3, n)?;
                machine.input_pointer += 1;
            }
            Op::Output => {
                machine.output_buffer.push(get_param(1)?);
            }
            Op::JNZ => {
                let p1 = get_param(1)?;
                let target = get_param(2)?;
                return if p1 != 0 {
                    machine.ip = target as usize;
                    Ok(0)
                } else {
                    Ok(self.len())
                };
            }
            Op::JZ => {
                let p1 = get_param(1)?;
                let target = get_param(2)?;
                return if p1 == 0 {
                    machine.ip = target as usize;
                    Ok(0)
                } else {
                    Ok(self.len())
                };
            }
            Op::LT => {
                let p1 = get_param(1)?;
                let p2 = get_param(2)?;

                write_val(machine, machine.ip + 3, to_num(p1 < p2))?;
            }
            Op::EQ => {
                let p1 = get_param(1)?;
                let p2 = get_param(2)?;

                write_val(machine, machine.ip + 3, to_num(p1 == p2))?;
            }
            Op::GT => {
                let p1 = get_param(1)?;
                let p2 = get_param(2)?;

                write_val(machine, machine.ip + 3, to_num(p1 > p2))?;
            }
            Op::JumpA => {
                unimplemented!()
                /*let p1 = get_param(1)?;
                let unsigned_p1 = p1.abs() as usize;
                if unsigned_p1 < machine.memory.len() {
                    machine.ip = unsigned_p1;
                } else {
                    return Err(ErrorStatus::OutOfBounds);
                }
                return Ok(0);*/
            }
            Op::JumpR => {
                unimplemented!()
                // Note that jumping with a negative offset requires
                // casting to a usize. We can't cast ip to isize or we
                // lose any values past isize limit, so we have to
                // do this, instead.
                /*let p1 = get_param(1)?;
                let unsigned_p1 = p1.abs() as usize;
                if p1 < 0 {
                    machine.ip -= unsigned_p1;
                } else {
                    machine.ip -= unsigned_p1;
                }
                return Ok(0);*/
            }
            Op::Halt => machine.halt = true,
        }
        Ok(self.len())
    }

    pub fn len(&self) -> usize {
        match self {
            Op::Add => 4,
            Op::Mul => 4,
            Op::Input => 2,
            Op::Output => 2,
            Op::JNZ => 3,
            Op::JZ => 3,
            Op::LT => 4,
            Op::EQ => 4,
            Op::GT => 4,
            Op::JumpA => 2,
            Op::JumpR => 2,
            Op::Halt => 1,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct IntCodeMachine {
    pub memory: Vec<Num>,
    pub input_pointer: usize,
    pub input_buffer: Vec<Num>,
    pub output_buffer: Vec<Num>,
    pub ip: usize,
    pub halt: bool,
}

impl IntCodeMachine {
    pub fn new(memory: Vec<Num>, input_buffer: Vec<Num>) -> IntCodeMachine {
        IntCodeMachine {
            ip: 0,
            input_pointer: 0,
            input_buffer,
            output_buffer: vec![],
            memory,
            halt: false,
        }
    }
    pub fn execute(&mut self) -> Result<(), ErrorStatus> {
        // Re-use `modes` rather than create a new one every
        // time in order to avoid memory thrashing.
        let mut modes: Vec<ParamMode> = Vec::with_capacity(5);
        let mut params: Vec<Num> = vec![];
        //        println!("{:?}", self.memory);
        //        println!("Running...");
        loop {
            if self.ip >= self.memory.len() {
                return Err(ErrorStatus::UnterminatedProgram);
            }
            modes.clear(); // important
            let op = deconstruct_opcode(
                *self.memory.get(self.ip).ok_or(ErrorStatus::OutOfBounds)?,
                &mut modes,
            )?;
            params.clear();
            if op.len() > 1 {
                for i in 0..(op.len() - 1) {
                    params.push(
                        *self
                            .memory
                            .get(self.ip + i + 1)
                            .ok_or(ErrorStatus::OutOfBounds)?,
                    );
                }
            }
            let cons = op.apply(&modes, self)?;
            //            println!("{:04} :: {:?}, Modes: {:?}, Params: {:?}", cycle, op, modes, params);
            //            println!("{:?}", &self.memory);
            if self.halt {
                //                println!("{:?}", self.memory);
                //                println!("Finished");
                break Ok(());
            }
            self.ip += cons;
        }
    }
}

// opcode is in format EDCBA where BA is 2 digit opcode, C is first
// parameter mode, D is second, E is third, etc (flipped from AoC).
// Note that currently we only support 0 for position and 1 for immediate.
// We use bit masks currently to detect whether immediate mode is
// active for a given parameter.
// Potential TODO: support more modes and params.
// DOES NOT CLEAR MODES. We do that explicitly up there ^ for separation of concerns
fn deconstruct_opcode(opcode: Num, modes: &mut Vec<ParamMode>) -> Result<Op, ErrorStatus> {
    let op = opcode - ((opcode / 100) * 100);

    let mut o = opcode / 100;
    while o > 0 {
        if o % 10 == 1 {
            modes.push(ParamMode::Immediate);
        } else {
            modes.push(ParamMode::Position);
        }
        o = o / 10;
    }

    // ^ won't detect implicit leading zeros, so we push until we have minimum parameters.
    // alternatively, I could refactor my apply() logic to handle missing parameter options
    while modes.len() < 3 {
        modes.push(ParamMode::Position);
    }

    Op::new(op)
}

pub fn parse_intcode(s: &str) -> Result<Vec<Num>, std::string::ParseError> {
    let mut intcode = vec![];
    for v in s.trim().split(',') {
        intcode.push(v.parse::<Num>().unwrap());
    }
    Ok(intcode)
}
