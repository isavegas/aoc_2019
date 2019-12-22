use crate::AoCDay;
use lazy_static::lazy_static;
use std::collections::HashMap;

pub struct Day14;

const INPUT: &'static str = include_str!("../input/day_14.txt");

type ID = u8;

const ORE_ID: ID = 0;
const FUEL_ID: ID = 1;

// HashMap<component: &str, Vec<(output_count: usize, requirements: Vec<(amount: usize, component: &str)>>
type BackMap = HashMap<ID, Vec<(usize, Vec<(usize, ID)>)>>;
lazy_static! {
    static ref REACTIONS: BackMap = {
        let mut map = HashMap::new();
        let mut lookup: HashMap<&str, ID> = HashMap::new();
        lookup.insert("ORE", ORE_ID);
        lookup.insert("FUEL", FUEL_ID);
        for l in INPUT.trim().lines() {
            let arr = l.find("=>").unwrap();
            let mut split = l[arr+2..].trim().split(' ');
            let amount: usize = split.next().unwrap().parse().unwrap();
            let len = lookup.len();
            let out = *lookup.entry(split.next().unwrap()).or_insert(len as ID);
            let components: Vec<(usize, ID)> = l[0..arr].split(',').map(|c| c.trim()).map(|c| {
                let mut split = c.split(' ');
                let len = lookup.len();
                (split.next().unwrap().parse::<usize>().unwrap(), *lookup.entry(split.next().unwrap().trim()).or_insert(len as ID))
            }).collect();
            map.entry(out).or_insert(vec![]).push((amount, components));
        }
        map
    };
}

fn cost(component: ID, amount: usize, mut store: HashMap<ID, usize>) -> (usize, HashMap<ID, usize>) {
    *store.entry(254).or_insert(0) +=  1;
    if component == 0 { return (amount, store); }
    let mut adj_amount = amount;
    let mut rem = false;
    if let Some(a) = store.get_mut(&component) {
        if adj_amount > *a {
            adj_amount -= *a;
            *a = 0;
        } else {
            *a -= adj_amount;
            adj_amount = 0;
        }
        if *a == 0 { rem = true; }
    }
    if rem { store.remove(&component); }
    if adj_amount == 0 { return (0, store); }
    let reactions = REACTIONS.get(&component).unwrap();

/*    fn react((usize, Vec<(u8, usize)>)) {
        
    } */

    if reactions.len() == 1 {
        let (output, requirements) = &reactions[0];
        let mut batches = adj_amount / output;
        if adj_amount % output > 0 {
            batches += 1;
            *store.entry(component).or_insert(0) +=  output - (adj_amount % output);
        }
        let mut sum = 0;
        for (req_amount, req_component) in requirements.iter() {
            let r = cost(*req_component, (*req_amount) * batches, store);
            store = r.1;
            sum += r.0;
        }
        (sum, store)
    } else {
        reactions.iter().map(|(output, requirements)| {
            let mut c_store = store.clone();
            *c_store.entry(255).or_insert(0) +=  1;
            let mut batches = adj_amount / output;
            if adj_amount % output > 0 {
                batches += 1;
                *c_store.entry(component).or_insert(0) +=  output - (adj_amount % output);
            }
            let mut sum = 0;
            for (req_amount, req_component) in requirements.iter() {
                let r = cost(*req_component, (*req_amount) * batches, c_store);
                c_store = r.1;
                sum += r.0;
            }
            (sum, c_store)
        }).min_by_key(|(c, _)| *c).unwrap()
    }
}

impl AoCDay for Day14 {
    fn day(&self) -> usize {
        14
    }
    fn part1(&self) -> String {
        let out = cost(FUEL_ID, 1, HashMap::new());
        println!("cost calls: {}", out.1.get(&254).unwrap_or(&0));
        println!("Forks: {}", out.1.get(&255).unwrap_or(&0));
        format!("{}", out.0)
    }
    fn part2(&self) -> String {
        let mut ore_count = 1_000_000_000_000;
        let mut store = HashMap::new();
        let mut amount = 0;
        while ore_count > 0 {
            let r = cost(FUEL_ID, 1, store);
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
