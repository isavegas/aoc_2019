use crate::AoCDay;
use lazy_static::lazy_static;
use std::collections::HashMap;

pub struct Day14;

const INPUT: &'static str = include_str!("../input/day_14.txt");

type BackMap = HashMap<&'static str, Vec<(usize, Vec<(usize, &'static str)>)>>;
lazy_static! {
    static ref REACTIONS: BackMap = {
        let mut map = HashMap::new();
        for l in INPUT.trim().lines() {
            let arr = l.find("=>").unwrap();
            let mut split = l[arr+2..].trim().split(' ');
            let amount: usize = split.next().unwrap().parse().unwrap();
            let out = split.next().unwrap();
            let components: Vec<(usize, &'static str)> = l[0..arr].split(',').map(|c| c.trim()).map(|c| {
                let mut split = c.split(' ');
                (split.next().unwrap().parse::<usize>().unwrap(), split.next().unwrap().trim())
            }).collect();
            map.entry(out).or_insert(vec![]).push((amount, components));
        }
        map
    };
}

#[derive(Debug, Clone, PartialEq)]
struct Reaction {
    output: (usize, String),
    requirements: Vec<(usize, String)>,
}

fn cost(component: &'static str, amount: usize, mut store: HashMap<&'static str, usize>) -> (usize, HashMap<&'static str, usize>) {
    if component == "ORE" { return (amount, store); }
    let mut adj_amount = amount;
    let mut rem = false;
    if let Some(a) = store.get_mut(component) {
        if adj_amount > *a {
            adj_amount -= *a;
            *a = 0;
        } else {
            *a -= adj_amount;
            adj_amount = 0;
        }
        if *a == 0 { rem = true; }
    }
    if rem {store.remove(component);}
    if adj_amount == 0 { return (0, store); }
    REACTIONS.get(component).unwrap().iter().map(|(output, requirements)| {
        let mut batches = adj_amount / output;
        let mut c_store = store.clone();
        if adj_amount % output > 0 {
            batches += 1;
            *c_store.entry(component).or_insert(0) +=  output - (adj_amount % output);
        }
        let mut sum = 0;
        for (req_amount, req_component) in requirements.iter() {
            let r = cost(req_component, (*req_amount) * batches, c_store);
            c_store = r.1;
            sum += r.0;
        }
        (sum, c_store)
    }).min_by_key(|(c, cs)| *c).unwrap()
}

impl AoCDay for Day14 {
    fn day(&self) -> usize {
        14
    }
    fn part1(&self) -> String {
        format!("{}", cost("FUEL", 1, HashMap::new()).0)
    }
    fn part2(&self) -> String {
        let mut ore_count = 1_000_000_000_000;
        let mut store = HashMap::new();
        let mut amount = 0;
        while ore_count > 0 {
            let r = cost("FUEL", 1, store);
            if ore_count > r.0 {
                ore_count -= r.0;
                store = r.1;
                amount += 1;
            } else {
                break;
            }
        }
        format!("{}", amount)
    }
}

pub fn get_day() -> Box<dyn AoCDay> {
    Box::new(Day14)
}
