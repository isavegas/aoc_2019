
use crate::AoCDay;

pub struct Day4;

impl AoCDay for Day4 {
    fn day(&self) -> i32 {
        4
    }
    fn part1(&self) -> String {
        unimplemented!()
    }
    fn part2(&self) -> String {
        unimplemented!()
    }
}

pub fn get_day() -> Box<dyn AoCDay> {
    Box::new(Day4)
}
