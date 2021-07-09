pub fn block_char() -> char {
    std::char::from_u32(9608).unwrap()
}

mod day;
pub use day::get_days;
