use crate::AoCDay;
use std::collections::HashMap;

pub struct Day06;

const INPUT: &'static str = include_str!("./input/day_06.txt");

fn build_chain(
    target: &'static str,
    map: &HashMap<&'static str, &'static str>,
    v: &mut Vec<&'static str>,
) {
    let mut last = target;
    while last != "COM" {
        last = map.get(last).unwrap();
        v.push(last);
    }
}

impl AoCDay for Day06 {
    fn day(&self) -> i32 {
        06
    }
    fn part1(&self) -> String {
        let mut orbit_map: HashMap<&'static str, &'static str> = HashMap::new();
        for s in INPUT.trim().split('\n') {
            let v = s.split(')').collect::<Vec<&'static str>>();
            orbit_map.insert(v[1], v[0]);
        }
        let mut orbits = 0;
        let mut cache: Vec<&'static str> = vec![];
        for key in orbit_map.keys() {
            cache.clear();
            build_chain(key, &orbit_map, &mut cache);
            orbits += cache.len();
        }
        format!("{}", orbits)
    }
    fn part2(&self) -> String {
        let mut orbit_map: HashMap<&'static str, &'static str> = HashMap::new();
        for s in INPUT.trim().split('\n') {
            let v = s.split(')').collect::<Vec<&'static str>>();
            orbit_map.insert(v[1], v[0]);
        }
        let mut start: Vec<&'static str> = vec![];
        build_chain("YOU", &orbit_map, &mut start);
        let mut end: Vec<&'static str> = vec![];
        build_chain("SAN", &orbit_map, &mut end);

        for (i, n) in start.iter().enumerate() {
            if let Some(i2) = end.iter().position(|f| f == n) {
                return format!("{}", i + i2);
            }
        }
        format!("Unable to find common orbit")
    }
}

pub fn get_day() -> Box<dyn AoCDay> {
    Box::new(Day06)
}
