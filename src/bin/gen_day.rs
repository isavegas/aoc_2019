macro_rules! template {
    () => (r#"use crate::AoCDay;

pub struct Day{0};

type Num = usize;

const INPUT: &'static str = include_str!("./input/day_{0}.txt");

impl AoCDay for Day{0} {{
    fn day(&self) -> i32 {{
        {0}
    }}
    fn part1(&self) -> String {{
        unimplemented!()
    }}
    fn part2(&self) -> String {{
        unimplemented!()
    }}
}}

pub fn get_day() -> Box<dyn AoCDay> {{
    Box::new(Day{0})
}}
"#;)
}

use std::fs::OpenOptions;
use std::io::{Write, BufWriter};

pub fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() == 2 {
        if let Ok(num) = args[1].parse::<usize>() {
            let num_str = format!("{:02}", num);
            let day_path = format!("{}/day_{}.rs", concat!(env!("CARGO_MANIFEST_DIR"), "/src/day"), num_str);
            let mut f = BufWriter::new(OpenOptions::new().create_new(true).write(true).open(day_path.clone()).expect("Unable to create file"));
            write!(f, template!(), num_str).expect("Unable to write file");
            println!("Output file created at {}", day_path);
        }
    }
}