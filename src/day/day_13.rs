use crate::AoCDay;
use crate::intcode::{ IntCodeMachine, ExecutionStatus, ErrorStatus, Num, parse_intcode };
use lazy_static::lazy_static;

pub struct Day13;

const INPUT: &'static str = include_str!("../input/day_13.txt");
lazy_static! {
    static ref INTCODE: Vec<Num> = parse_intcode(INPUT).expect("Unable to parse bundled intcode");
}

#[derive(PartialEq, Eq, Clone, Debug)]
enum Tile {
    Empty,
    Block,
    Paddle,
    Ball,
    Wall,
}

impl Tile {
    fn from(n: Num) -> Tile {
        match n {
            0 => Tile::Empty,
            1 => Tile::Wall,
            2 => Tile::Block,
            3 => Tile::Paddle,
            4 => Tile::Ball,
            _ => panic!("Undefined tile!"),
        }
    }
}

impl AoCDay for Day13 {
    fn day(&self) -> usize {
        13
    }
    fn part1(&self) -> String {
        let mut machine = IntCodeMachine::new(INTCODE.clone(), vec![], 100);
        machine.execute().expect("Machine has crashed!");
        format!("{}", machine.output_buffer.iter().skip(2).step_by(3).filter(|b| *b == &2).count())
    }
    fn part2(&self) -> String {
        let mut intcode = INTCODE.clone();
        intcode[0] = 2;
        let mut machine = IntCodeMachine::new(intcode, vec![], 1000);

        struct State {
            score: Num,
            paddle_pos: Num,
            ball_pos: Num,
        }
        let mut state = State {
            score: 0,
            ball_pos: 0,
            paddle_pos: 0,
        };

        fn handle_output(machine: &mut IntCodeMachine, state: &mut State) {
            while machine.output_buffer.len() > 0 {
                let command = machine.output_buffer.pop().expect("Expected a command");
                let y = machine.output_buffer.pop().expect("Expected a y value");
                let x = machine.output_buffer.pop().expect("Expected an x value");
                if y == 0 && x == -1 {
                    state.score = command;
                } else {
                    match Tile::from(command) {
                                Tile::Ball => state.ball_pos = x,
                                Tile::Paddle => state.paddle_pos = x,
                                _ => (),
                            }
                        }
                    }
                    if state.ball_pos > state.paddle_pos {
                        machine.input_buffer.push(1);
                    } else if state.ball_pos < state.paddle_pos {
                        machine.input_buffer.push(-1);
                    } else {
                        machine.input_buffer.push(0);
                    }
        }
        loop {
            match machine.execute().expect("Machine has crashed!") {
                ExecutionStatus::Halted => {
                    handle_output(&mut machine, &mut state);
                    break;
                },
                ExecutionStatus::Blocking => handle_output(&mut machine, &mut state)
            }
        }
        format!("{}", state.score)
    }
}

pub fn get_day() -> Box<dyn AoCDay> {
    Box::new(Day13)
}
