pub trait AoCDay {
    fn day(&self) -> usize;
    fn part1(&self) -> String;
    fn part2(&self) -> String;
}

pub fn block_char() -> char {
    std::char::from_u32(9608).unwrap()
}

mod vec3;
pub use vec3::Vec3;
mod vec2;
pub use vec2::Vec2;

mod error;
pub use error::ErrorWrapper;

mod day;
pub mod intcode;
pub use day::get_days;

pub mod math;
