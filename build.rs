#![feature(option_result_contains)]

use std::env;
use std::fs::{read_dir, File};
use std::io::Write;
use std::path::Path;

macro_rules! get_days_template {
    () => {
        r#"
use aoc_core::AoCDay;

pub fn get_days() -> Vec<Box<dyn AoCDay>> {{
    let mut days = vec![
        {}
    ];
    // Enforce sorting by day at runtime
    days.sort_by_key(|d| d.day());
    days
}}
"#
    };
}

pub fn main() {
    let in_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    let days_path = Path::new(&in_dir).join("src/day/");
    let days: Vec<String> = read_dir(days_path)
        .expect("Unable to access $CARGO_MANIFEST_DIR/src/day/")
        .map(|e| {
            e.expect("Error occured while iterating $CARGO_MANIFEST_DIR/src/day")
                .path()
        })
        .filter(|e| e.is_file())
        .filter(|e| e.extension().contains(&"rs"))
        .map(|e| {
            e.file_stem()
                .unwrap()
                .to_str()
                .expect("Invalid filename")
                .to_string()
        })
        .filter(|f| f.starts_with("day_"))
        .collect();

    let out_dir = env::var("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("generated_get_days.rs");
    let mut f = File::create(&dest_path).unwrap();

    let returns: String = days
        .iter()
        .map(|d| format!("crate::day::{}::get_day(),\n", d))
        .collect();

    f.write_all(format!(get_days_template!(), returns).as_bytes())
        .unwrap();
}
