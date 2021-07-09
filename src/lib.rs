pub trait AoCDay {
    fn day(&self) -> usize;
    fn part1(&self) -> Result<String, ErrorWrapper>;
    fn part2(&self) -> Result<String, ErrorWrapper>;
    fn expected(&self) -> (Option<&'static str>, Option<&'static str>);
}

pub fn block_char() -> char {
    std::char::from_u32(9608).unwrap()
}

pub use aoc_core::*;

mod day;
//pub mod intcode;
pub use day::get_days;
