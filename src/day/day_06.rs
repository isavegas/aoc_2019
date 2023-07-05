use aoc_core::{bail, AoCDay, ErrorWrapper};
use std::collections::HashMap;

pub struct Day06;

fn build_chain(target: &str, map: &HashMap<String, String>, v: &mut Vec<String>) {
    let mut last = target;
    while last != "COM" {
        last = map.get(last).unwrap();
        v.push(last.to_string());
    }
}

impl AoCDay for Day06 {
    fn day(&self) -> usize {
        6
    }
    fn expected(&self) -> (Option<&'static str>, Option<&'static str>) {
        (None, None)
    }
    fn part1(&self, input: &str) -> Result<String, ErrorWrapper> {
        let mut orbit_map: HashMap<String, String> = HashMap::new();
        for s in input.trim().split('\n') {
            let v = s.split(')').collect::<Vec<&str>>();
            orbit_map.insert(v[1].to_string(), v[0].to_string());
        }
        let mut orbits = 0;
        let mut cache: Vec<String> = vec![];
        for key in orbit_map.keys() {
            cache.clear();
            build_chain(key, &orbit_map, &mut cache);
            orbits += cache.len();
        }
        Ok(format!("{}", orbits))
    }
    fn part2(&self, input: &str) -> Result<String, ErrorWrapper> {
        let mut orbit_map: HashMap<String, String> = HashMap::new();
        for s in input.trim().split('\n') {
            let v = s.split(')').collect::<Vec<&str>>();
            orbit_map.insert(v[1].to_string(), v[0].to_string());
        }
        let mut start: Vec<String> = vec![];
        build_chain("YOU", &orbit_map, &mut start);
        let mut end: Vec<String> = vec![];
        build_chain("SAN", &orbit_map, &mut end);

        for (i, n) in start.iter().enumerate() {
            if let Some(i2) = end.iter().position(|f| f == n) {
                return Ok(format!("{}", i + i2));
            }
        }
        bail!("Unable to find common orbit")
    }
}

pub fn get_day() -> Box<dyn AoCDay> {
    Box::new(Day06)
}
