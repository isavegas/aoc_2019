use aoc_core::{AoCDay, ErrorWrapper};
pub struct Day04;

type Num = usize;

fn adjacent(n: Num) -> bool {
    let n_str = format!("{}", n);
    let mut last = n_str.chars().next().unwrap();
    for c in n_str.chars().skip(1) {
        if c == last {
            return true;
        } else {
            last = c;
        }
    }
    false
}

fn adjacent_strict(n: Num) -> bool {
    let chars: Vec<char> = format!("{:06}", n).chars().collect();
    if chars[0] == chars[1] && chars[1] != chars[2] {
        return true;
    }
    if chars[chars.len() - 3] != chars[chars.len() - 2]
        && chars[chars.len() - 2] == chars[chars.len() - 1]
    {
        return true;
    }

    for i in 4..=chars.len() {
        let block = [chars[i - 4], chars[i - 3], chars[i - 2], chars[i - 1]];
        if block[0] != block[1] && block[1] == block[2] && block[2] != block[3] {
            return true;
        }
    }
    false
}

fn decreasing(n: Num) -> bool {
    let n_str = format!("{}", n);
    let mut last: Num = n_str.chars().next().unwrap().to_digit(10).unwrap() as Num;
    for c in n_str.chars().skip(1) {
        let c_n = c.to_digit(10).unwrap() as Num;
        if c_n < last {
            return true;
        } else {
            last = c_n;
        }
    }
    false
}

impl AoCDay for Day04 {
    fn day(&self) -> usize {
        4
    }
    fn expected(&self) -> (Option<&'static str>, Option<&'static str>) {
        (None, None)
    }
    fn part1(&self, input: &str) -> Result<String, ErrorWrapper> {
        let (a, b) = parse_input(input);
        let mut found = 0;
        for n in a..b {
            if adjacent(n) && !decreasing(n) {
                found += 1
            }
        }
        Ok(format!("{}", found))
    }
    fn part2(&self, input: &str) -> Result<String, ErrorWrapper> {
        let (a, b) = parse_input(input);
        let mut found = 0;
        for n in a..b {
            if adjacent_strict(n) && !decreasing(n) {
                found += 1
            }
        }
        Ok(format!("{}", found))
    }
}

pub fn get_day() -> Box<dyn AoCDay> {
    Box::new(Day04)
}

fn parse_input(input: &str) -> (Num, Num) {
    let mut iter = input.trim().split('-');
    let a = iter.next().unwrap();
    let b = iter.next().unwrap();
    (
        a.parse().unwrap(),
        b.parse().unwrap(),
    )
}
