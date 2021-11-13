pub fn block_char() -> char {
    std::char::from_u32(9608).unwrap()
}

pub mod day;
pub use day::get_days;

mod input;
pub use input::get_inputs;
