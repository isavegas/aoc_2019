use crate::AoCDay;
use lazy_static::lazy_static;
use std::collections::HashMap;
use std::hash::Hasher;

pub struct Day10;

const INPUT: &'static str = include_str!("../input/day_10.txt");

lazy_static! {
    static ref POINTS: Vec<Point> = {
        let map: Vec<Vec<bool>> = INPUT
            .trim()
            .lines()
            .map(|l| l
                .chars()
                .map(|c| match c { '#' => true, _ => false, })
                .collect())
            .collect();
        let width = map[0].len();
        let mut points = vec![];
        for y in 0..map.len() {
            for x in 0..width {
                if map[y][x] {
                    points.push(Point { x: x as i64, y: y as i64});
                }
            }
        }
        points
    };
}

use std::mem;
fn integer_decode(val: f64) -> (u64, i16, i8) {
    let bits: u64 = unsafe {
        mem::transmute(val)
    };
    let sign: i8 = if bits >> 63 == 0 {
        1
    } else {
        -1
    };
    let mut exponent: i16 = ((bits >> 52) & 0x7ff) as i16;
    let mantissa = if exponent == 0 {
        (bits & 0xfffffffffffff) << 1
    } else {
        (bits & 0xfffffffffffff) | 0x10000000000000
    };

    exponent -= 1023 + 52;
    (mantissa, exponent, sign)
}

#[derive(Debug, PartialEq, Clone, Eq, Hash)]
struct Point {
    x: i64,
    y: i64,
}

impl Point {
    fn distance(&self, o: &Point) -> f64 {
        (((self.x - o.x).abs().pow(2) + (self.y - o.y).abs().pow(2)) as f64).sqrt()
    }

    fn atan2(&self, o: &Point) -> f64 {
        (self.y as f64 - o.y as f64).atan2(self.x as f64 - o.x as f64)
    }

    fn collinear(&self, o1: &Point, o2: &Point) -> bool {
        self.atan2(o1) == self.atan2(o2)
    }
}

fn best_visibility(points: &[Point]) -> (&Point, usize) {
        let mut counts: Vec<(&Point, usize)> = vec![];

        for origin in points {
            // No idea why this is required for it to work
            let mut count = 1;
            for focus in points {
                if origin != focus {
                    let mut visible = true;
                    for check in points {
                        if focus != check {
                            if origin.distance(check) < origin.distance(focus) && origin.collinear(focus, check) {
                                visible = false;
                            }
                        }
                    }
                    if visible { count += 1; }
                }
            }
            counts.push((origin, count));
        }

        *counts.iter().max_by_key(|(_, c)| c).unwrap()
}

impl AoCDay for Day10 {
    fn day(&self) -> i32 {
        10
    }
    fn part1(&self) -> String {
        best_visibility(&POINTS).1.to_string()
    }
    fn part2(&self) -> String {
        let mut objects = POINTS.clone();
        let laser = {
            let b = best_visibility(&objects).0.clone();
            objects.remove(objects.iter().position(|r| r == &b).unwrap())
        };
        fn normalize(f: f64) -> f64 {
            (f + (2.0 * std::f64::consts::PI)) % (2.0 * std::f64::consts::PI)
        }

        let angles: HashMap<&Point, f64> = objects.iter().map(|o| (o, normalize(laser.atan2(o)))).collect();

        let mut quads: [Vec<(&Point, f64)>; 4] = [vec![], vec![], vec![], vec![]];
        //quads.sort_by(|(_, a), (_, b)| a.cmp(b));

        //let mut quad2: Vec<(&Point, Num)> = angles.iter().filter(|(p, f)| *f <= 0.5 * PI && *f > 0.0).map(|(p, f)| (*p, *f)).collect();
        //let mut quad3: Vec<(&Point, Num)> = angles.iter().filter(|(p, f)| *f <= 0.0 && *f > -0.5 * PI).map(|(p, f)| (*p, *f)).collect();
        //let mut quad4: Vec<(&Point, Num)> = angles.iter().filter(|(p, f)| *f < -0.5 * PI).map(|(p, f)| (*p, *f)).collect();
        //let mut quad1: Vec<(&Point, Num)> = angles.iter().filter(|(p, f)| *f > 0.5 * PI).map(|(p, f)| (*p, *f)).collect();

        println!("{:?}", angles);
        //let mut destroyed = vec![];
        loop {
            //for i in 0..quad2.len() {
            //
            //}
            break;
        }

        format!("{:?}", laser)
    }
}

pub fn get_day() -> Box<dyn AoCDay> {
    Box::new(Day10)
}
