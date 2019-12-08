use crate::AoCDay;

pub struct Day1;

impl AoCDay for Day1 {
    fn day(&self) -> i32 {
        1
    }
    fn part1(&self) -> String {
        unimplemented!()
    }
    fn part2(&self) -> String {
        unimplemented!()
    }
}

pub fn get_day() -> Box<dyn AoCDay> {
    Box::new(Day1)
}
