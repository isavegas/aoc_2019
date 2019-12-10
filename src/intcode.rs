use std::collections::HashMap;

pub type Num = isize;

#[inline(always)]
fn to_num(b: bool) -> Num {
    match b {
        true => 1,
        false => 0,
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum ErrorStatus {
    UnterminatedProgram,
    UnrecognizedOp(Num),
    OutOfBounds,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ParamMode {
    Immediate,
    Relative,
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
    ARB,
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
            9 => Ok(Op::ARB),
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
        macro_rules! mem_write {
            ($offset:expr, $value:expr) => (
                {
                    let o = $offset;
                    machine.memory.write(machine.ip + o, $value, modes[o - 1])
                }
            )
        }
        macro_rules! mem_read {
            ($offset:expr) => (
                {
                    let o = $offset;
                    machine.memory.read(machine.ip + o, modes[o - 1])
                }
            )
        }
        let mut params_f = vec![];
        for n in 1..self.len() {
            params_f.push(machine.memory.read_raw(machine.ip + n)?);
        }

        //println!("{} :: {:?} :: {:?} :: {:?}", machine.ip, self, params_f, modes);
        match self {
            Op::Add => {
                let p1 = mem_read!(1)?;
                let p2 = mem_read!(2)?;
                mem_write!(3, p1 + p2)?;
            }
            Op::Mul => {
                let p1 = mem_read!(1)?;
                let p2 = mem_read!(2)?;
                mem_write!(3, p1 * p2)?;
            }
            Op::Input => {
                if let Some(n) = machine.input_buffer.get(machine.input_pointer).cloned() {
                    mem_write!(1, n)?;
                    machine.input_pointer += 1;
                } else {
                    machine.blocking = true;
                    // Tell the VM not to increment IP
                    return Ok(0);
                }
            }
            Op::Output => {
                machine.output_buffer.push(mem_read!(1)?);
            }
            Op::JNZ => {
                let p1 = mem_read!(1)?;
                let target = mem_read!(2)?;
                match p1 != 0 {
                    true => {
                        machine.ip = target as usize;
                        return Ok(0);
                    },
                    false => {
                        return Ok(self.len());
                    },
                }
            }
            Op::JZ => {
                let p1 = mem_read!(1)?;
                let target = mem_read!(2)?;
                match p1 == 0 {
                    true => {
                        machine.ip = target as usize;
                        return Ok(0);
                    },
                    false => {
                        return Ok(self.len());
                    },
                }
            }
            Op::LT => {
                let p1 = mem_read!(1)?;
                let p2 = mem_read!(2)?;
                mem_write!(3, to_num(p1 < p2))?;
            }
            Op::EQ => {
                let p1 = mem_read!(1)?;
                let p2 = mem_read!(2)?;
                mem_write!(3, to_num(p1 == p2))?;
            }
            Op::ARB => {
                let p1 = mem_read!(1)?;
                machine.memory.adjust_relative_base(p1);
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
            Op::ARB => 2,
            Op::Halt => 1,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum ExecutionStatus {
    Blocking,
    Halted,
}

// We only support a single page size.
// TODO: Mixed page sizes
#[derive(Debug, Clone, PartialEq)]
pub struct Memory {
    pub page_size: usize,
    pub relative_base: usize,
    pub page_table: HashMap<usize, Vec<Num>>,
    pub last_page: usize,
}

// TODO: Propogate error gracefully if attempt to write to
// negative address through positional/relative params occurs.
impl Memory {
    pub fn from_vec(page_size: usize, data: Vec<Num>) -> Memory {
        let mut memory = Memory {
            page_size,
            relative_base: 0,
            page_table: HashMap::new(),
            last_page: 0,
        };
        let mut chunks = data.chunks_exact(memory.page_size);
        let mut i = 0;
        loop {
            if let Some(d) = chunks.next() {
                memory.page_table.insert(i, Vec::from(d));
                i += 1;
            } else {
                break;
            }
        }
        let remainder = chunks.remainder();
        if remainder.len() > 0 {
            let mut page = Vec::from(remainder);
            page.resize_with(page_size, || 0);
            memory.page_table.insert(i, page);
        }
        memory.last_page = i;
        memory
    }
    // returns (page index, page offset)
    #[inline(always)]
    fn resolve_virtual_address(&self, address: usize) -> (usize, usize) {
        (address / self.page_size, address % self.page_size)
    }
    #[inline(always)]
    fn relative_address(&self, offset: Num) -> usize {
        match offset < 0 {
            true => self.relative_base - offset.abs() as usize,
            false => self.relative_base + offset as usize,
        }
    }
    pub fn adjust_relative_base(&mut self, offset: Num) -> Result<(), ErrorStatus> {
                if offset < 0 {
                    self.relative_base -= offset.abs() as usize;
                } else {
                    self.relative_base += offset as usize;
                }
        Ok(())
    }
    // TODO: Add an out of memory error?
    pub fn write_raw(&mut self, address: usize, value: Num) -> Result<(), ErrorStatus> {
        // integer division
        let (page_index, page_offset) = self.resolve_virtual_address(address);
        if page_index > self.last_page {
            self.last_page = page_index;
        }
        let mut page = self
            .page_table
            .entry(address / self.page_size)
            .or_insert(vec![0; self.page_size]);
        page[page_offset] = value;
        Ok(())
    }

    pub fn write(
        &mut self,
        address: usize,
        value: Num,
        mode: ParamMode,
    ) -> Result<(), ErrorStatus> {
        // There isn't an immediate mode for writing. Either we have position or relative,
        // but the VM doesn't know or care about valid modes, as it simply propogates them to
        // the read/write functions.
        match mode {
            ParamMode::Immediate | ParamMode::Position => self.write_raw(self.read_raw(address)? as usize, value),
            ParamMode::Relative => {
                let relative_offset = self.read_raw(address)?;
                let address = self.relative_address(relative_offset);
                self.write_raw(address, value)
            }
        }
    }
    // We don't even bother allocating the memory page if it doesn't exist, as it will return 0 anyway.
    // We have to check if the page exists when attempting to access it anyway.
    pub fn read_raw(&self, address: usize) -> Result<Num, ErrorStatus> {
        let (page_index, page_offset) = self.resolve_virtual_address(address);
        match self.page_table.get(&page_index) {
            Some(page) => Ok(page[page_offset]),
            None => {
                //println!("Cache miss: {}", address);
                Ok(0)
            },
        }
    }
    pub fn read(&self, address: usize, mode: ParamMode) -> Result<Num, ErrorStatus> {
        match mode {
            ParamMode::Immediate => self.read_raw(address),
            ParamMode::Position => {
                let a = self.read_raw(address)? as usize;
                self.read_raw(a)
            },
            ParamMode::Relative => {
                let relative_offset = self.read_raw(address)?;
                let address = self.relative_address(relative_offset);
                self.read_raw(address)
            },
        }
    }
    pub fn size(&self) -> usize {
        self.page_table.len() * self.page_size
    }
    // This seems inefficient if we hit it a lot
    pub fn virtual_size(&self) -> usize {
        (self.last_page + 1) * self.page_size
        //(self.page_table.keys().max().unwrap_or_else(|| &0) + 1) * self.page_size
    }
    pub fn flatten(&self) -> Vec<Num> {
        let v_size = self.virtual_size();
        let mut out = Vec::with_capacity(v_size);
        for i in 0..=(v_size / self.page_size) {
            match self.page_table.get(&i) {
                Some(p) => out.extend_from_slice(p.as_slice()),
                None => out.resize_with(self.page_size * i, || 0),
            }
        }
        out
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct IntCodeMachine {
    pub memory: Memory,
    pub input_pointer: usize,
    pub input_buffer: Vec<Num>,
    pub output_buffer: Vec<Num>,
    pub ip: usize,
    pub blocking: bool,
    pub halt: bool,
    pub log_ops: bool,
    pub iteration: usize,
}

impl IntCodeMachine {
    pub fn new(code: Vec<Num>, input_buffer: Vec<Num>, page_size: usize) -> IntCodeMachine {
        IntCodeMachine {
            ip: 0,
            input_pointer: 0,
            input_buffer,
            output_buffer: vec![],
            memory: Memory::from_vec(page_size, code),
            blocking: false,
            halt: false,
            log_ops: false,
            iteration: 0,
        }
    }
    pub fn execute(&mut self) -> Result<ExecutionStatus, ErrorStatus> {
        // Re-use `modes` rather than create a new one every
        // time in order to avoid memory thrashing.
        self.blocking = false;
        let mut modes: Vec<ParamMode> = Vec::with_capacity(5);
        let mut params: Vec<Num> = vec![];

        loop {
            // There is no 0 opcode, so we can go ahead and
            // assume that the program is going to crash
            if self.ip >= self.memory.virtual_size() {
                return Err(ErrorStatus::UnterminatedProgram);
            }

            modes.clear(); // important
            let op = deconstruct_opcode(self.memory.read_raw(self.ip)?, &mut modes)?;
            params.clear();
            if op.len() > 1 {
                for i in 0..(op.len() - 1) {
                    params.push(self.memory.read_raw(self.ip + i + 1)?);
                }
            }
            let cons = op.apply(&modes, self)?;
            self.iteration += 1;
            self.ip += cons;
            if self.halt {
                break Ok(ExecutionStatus::Halted);
            }
            if self.blocking {
                break Ok(ExecutionStatus::Blocking);
            }
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
        } else if o % 10 == 2 {
            modes.push(ParamMode::Relative);
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
