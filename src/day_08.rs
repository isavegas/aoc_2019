use crate::AoCDay;

pub struct Day08;

const INPUT: &'static str = include_str!("./input/day_08.txt");

impl AoCDay for Day08 {
    fn day(&self) -> i32 {
        08
    }
    fn part1(&self) -> String {
        unimplemented!()
    }
    fn part2(&self) -> String {
        unimplemented!()
    }
}

pub fn get_day() -> Box<dyn AoCDay> {
    Box::new(Day08)
}
