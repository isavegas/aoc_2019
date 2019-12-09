pub trait AoCDay {
    fn day(&self) -> i32;
    fn part1(&self) -> String;
    fn part2(&self) -> String;
}

mod error;
pub use error::ErrorWrapper;

pub mod intcode;
mod day;
pub use day::get_days;
