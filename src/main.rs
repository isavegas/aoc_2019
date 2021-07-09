use aoc_core::AoCProject;

pub fn main() {
    let project = AoCProject::new(2019, env!("CARGO_PKG_VERSION").to_string(), None, "isavegas".to_string(), None);
    project.run(&aoc_2019::get_days(), &aoc_2019::get_inputs());
}