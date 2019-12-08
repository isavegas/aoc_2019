use crate::AoCDay;

pub struct Day04;

type Num = usize;

const INPUT: &'static str = "172930-683082";

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

#[derive(Debug)]
struct NumStr {
    num: Num,
    str: String,
}

impl AoCDay for Day04 {
    fn day(&self) -> i32 {
        04
    }
    fn part1(&self) -> String {
        let (a, b) = get_input();
        let mut found = 0;
        for n in a.num..b.num {
            if adjacent(n) && !decreasing(n) {
                found += 1
            }
        }
        format!("{}", found)
    }
    fn part2(&self) -> String {
        let (a, b) = get_input();
        let mut found = 0;
        for n in a.num..b.num {
            if adjacent_strict(n) && !decreasing(n) {
                found += 1
            }
        }
        format!("{}", found)
    }
}

pub fn get_day() -> Box<dyn AoCDay> {
    Box::new(Day04)
}

fn get_input() -> (NumStr, NumStr) {
    let mut iter = INPUT.split('-');
    let a = iter.next().unwrap();
    let b = iter.next().unwrap();
    (
        NumStr {
            num: a.parse().unwrap(),
            str: a.to_string(),
        },
        NumStr {
            num: b.parse().unwrap(),
            str: b.to_string(),
        },
    )
}
