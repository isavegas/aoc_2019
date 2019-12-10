use std::env;
use std::fs::{read_dir, File};
use std::io::Write;
use std::path::Path;

macro_rules! get_days_template {
    () => {
        r#"
use crate::AoCDay;

pub fn get_days() -> Vec<Box<dyn AoCDay>> {{
    vec![
        {}
    ]
}}
"#
    };
}

pub fn main() {
    let in_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    let days_path = Path::new(&in_dir).join("src/day/");
    let days: Vec<String> = read_dir(days_path)
        .unwrap()
        .map(|e| e.unwrap().path())
        .filter(|e| e.is_file() && e.extension().is_some() && e.extension().unwrap() == "rs")
        .map(|e| e.file_stem().unwrap().to_str().unwrap().to_string())
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
