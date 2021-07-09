macro_rules! template {
    () => {
        r#"use crate::*;

use lazy_static::lazy_static;

pub struct Day{0};

type Num = u64;
const INPUT: &'static str = include_str!("../input/day_{0}.txt");

lazy_static! {{
    static ref DATA: Vec<Num> = crate::parse::numbers(INPUT);
}}

impl AoCDay for Day{0} {{
    fn day(&self) -> usize {{
        {0}
    }}
    fn expected(&self) -> (Option<&'static str>, Option<&'static str>) {{
        (None, None)
    }}
    fn part1(&self) -> Result<String, ErrorWrapper> {{
        Err(ErrorWrapper::NotImplemented)
    }}
    fn part2(&self) -> Result<String, ErrorWrapper> {{
        Err(ErrorWrapper::NotImplemented)
    }}
}}

pub fn get_day() -> Box<dyn AoCDay> {{
    Box::new(Day{0})
}}
"#;
    };
}

use std::fs::OpenOptions;
use std::io::{BufWriter, Write};

pub fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() == 2 {
        if let Ok(num) = args[1].parse::<usize>() {
            let num_str = format!("{:02}", num);
            let day_path = format!(
                "{}/day_{}.rs",
                concat!(env!("CARGO_MANIFEST_DIR"), "/src/day"),
                num_str
            );
            let input_path = format!(
                "{}/day_{}.txt",
                concat!(env!("CARGO_MANIFEST_DIR"), "/src/input"),
                num_str
            );
            let mut f = BufWriter::new(
                OpenOptions::new()
                    .create_new(true)
                    .write(true)
                    .open(day_path.clone())
                    .expect("Unable to create file"),
            );
            write!(f, template!(), num_str).expect("Unable to write file");
            println!("Output file created at {}", day_path);
            OpenOptions::new()
                .create_new(true)
                .write(true)
                .open(input_path.clone())
                .expect("Unable to create input file");
            println!("Input file created at {}", input_path);
        }
    }
}
