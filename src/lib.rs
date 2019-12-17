pub trait AoCDay {
    fn day(&self) -> usize;
    fn part1(&self) -> String;
    fn part2(&self) -> String;
}

mod vec3;
pub use vec3::Vec3;

mod error;
pub use error::ErrorWrapper;

mod day;
pub mod intcode;
pub use day::get_days;
