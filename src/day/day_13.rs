use crate::AoCDay;

pub struct Day13;

const INPUT: &'static str = include_str!("../input/day_13.txt");

impl AoCDay for Day13 {
    fn day(&self) -> usize {
        13
    }
    fn part1(&self) -> String {
        unimplemented!()
    }
    fn part2(&self) -> String {
        unimplemented!()
    }
}

pub fn get_day() -> Box<dyn AoCDay> {
    Box::new(Day13)
}
