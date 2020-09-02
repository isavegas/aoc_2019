use crate::AoCDay;
use lazy_static::lazy_static;

pub struct Day1;

const INPUT: &'static str = include_str!("../input/day_01.txt");
type Num = u64;

fn parse(input: &str) -> Vec<Num> {
    input
        .lines()
        .map(str::trim)
        .filter(|l| l.len() > 0)
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
    fn part1(&self) -> String {
        format!(
            "{}",
            FUEL.iter()
                .map(calc)
                .sum::<Num>()
        )
    }
    fn part2(&self) -> String {
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
        format!("{}", total)
    }
}

pub fn get_day() -> Box<dyn AoCDay> {
    Box::new(Day1)
}
