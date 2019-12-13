use aoc_2019::intcode::{parse_intcode, IntCodeMachine, Num};
use aoc_2019::AoCDay;
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use lazy_static::lazy_static;

pub struct Day02;

const INPUT: &'static str = include_str!("../src/input/day_09.txt");

lazy_static! {
    // This should ALWAYS succeed.
    static ref INTCODE: Vec<Num> = parse_intcode(INPUT).expect("Invalid intcode bundled into application");
}

#[inline(never)]
pub fn day_09_part_1(f: usize) {
    let mut machine = IntCodeMachine::new(INTCODE.clone(), vec![1], f);
    let err = machine.execute();
    if err.is_err() {
        println!("Error running machine! {:?}", err);
    }
}

#[inline(never)]
pub fn day_09_part_2(f: usize) {
    let mut machine = IntCodeMachine::new(INTCODE.clone(), vec![2], f);
    let err = machine.execute();
    if err.is_err() {
        println!("Error running machine! {:?}", err);
    }
}

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("Day 9, Part 1", |b| {
        b.iter(|| black_box(day_09_part_1(black_box(1000))))
    });
    c.bench_function("Day 9, Part 2", |b| {
        b.iter(|| black_box(day_09_part_2(black_box(1000))))
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
