use aoc_core::{AoCDay, ErrorWrapper};
use lazy_static::lazy_static;

pub struct Day1;

const INPUT: &str = include_str!("../input/day_01.txt");
type Num = u64;

fn parse(input: &str) -> Vec<Num> {
    input
        .lines()
        .map(str::trim)
        .filter(|l| !l.is_empty())
        .map(str::parse)
        .map(Result::unwrap)
        .collect()
}

lazy_static! {
    static ref FUEL: Vec<Num> = parse(INPUT);
}

fn calc(n: &Num) -> Num {
    let n2 = n / 3;
    if n2 <= 2 {
        0
    } else {
        n2 - 2
    }
}

impl AoCDay for Day1 {
    fn day(&self) -> usize {
        1
    }

    fn expected(&self) -> (Option<&'static str>, Option<&'static str>) {
        (Some("3452245"), Some("5175499"))
    }

    fn part1(&self) -> Result<String, ErrorWrapper> {
        Ok(FUEL.iter()
            .map(calc)
            .sum::<Num>()
            .to_string()
        )
    }

    fn part2(&self) -> Result<String, ErrorWrapper> {
        let mut total: Num = 0;
        let mut current = FUEL.clone();
        loop {
            for f in current.iter_mut().filter(|f| *f > &mut 0) {
                *f = calc(f);
            }
            let s: Num = current.iter().sum();
            if s > 0 {
                total += s;
            } else {
                break;
            }
        }
        Ok(total.to_string())
    }
}

pub fn get_day() -> Box<dyn AoCDay> {
    Box::new(Day1)
}
