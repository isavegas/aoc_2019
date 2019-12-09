pub trait AoCDay {
    fn day(&self) -> i32;
    fn part1(&self) -> String;
    fn part2(&self) -> String;
}

mod error;
pub use error::ErrorWrapper;

pub mod intcode;

mod day_01;
mod day_02;
mod day_03;
mod day_04;
mod day_05;
mod day_06;
mod day_07;

pub fn get_days() -> Vec<Box<dyn AoCDay>> {
    vec![
        day_01::get_day(),
        day_02::get_day(),
        day_03::get_day(),
        day_04::get_day(),
        day_05::get_day(),
        day_06::get_day(),
        day_07::get_day(),
    ]
}
