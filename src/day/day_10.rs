#![allow(unused_variables)] 
#![allow(unused_imports)] 
use crate::AoCDay;
use lazy_static::lazy_static;

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
    static ref BEST: (Point, usize) = {
        let (b, a) = best_visibility(&POINTS);
        (b.clone(), a)
    };
}

#[derive(Debug, PartialEq, Clone, Eq, Hash)]
struct Point {
    x: i64,
    y: i64,
}

impl Point {
    fn distance(&self, o: &Point) -> f64 {
        (((o.x - self.x).pow(2) + (o.y - self.y).pow(2)) as f64).sqrt()
    }

    fn atan2(&self, o: &Point) -> f64 {
        ((o.y - self.y) as f64).atan2((o.y - self.y) as f64)
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
                                break;
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
    fn day(&self) -> usize {
        10
    }
    fn part1(&self) -> String {
        BEST.1.to_string()
    }
    fn part2(&self) -> String {
        let mut objects = POINTS.clone();
        // Temporary
        let laser = Point { x: 26, y: 36 };
        objects.remove(objects.iter().position(|r| r == &laser).unwrap());
        /*let laser = {
            let b = best_visibility(&objects).0.clone();
            objects.remove(objects.iter().position(|r| r == &b).unwrap())
        };*/
        use std::f64::consts::PI;
        let vert_neg = PI * (3.0/2.0);
        let vert_pos = PI / 2.0;
        let hori_pos = PI*2.0;
        let hori_neg = PI;
        // Adjust to rotate with 0 being up, clockwise.
        // This is to allow us to simply sort by the angle and iterate
        // through the list over and over, checking for collisions and
        // destroying the asteroids that are visible. Flipping the sign
        // changes rotation direction and changing the value rotates around
        // the origin point
        fn normalize(f: f64) -> f64 {
            (f + (PI * 2.5)) % (PI * 2.0)
        }

        let mut angles: Vec<(&Point, f64)> = objects.iter().map(|o| (o, normalize(laser.atan2(o)))).collect();
        angles.sort_by(|(a, b), (a2, b2)| b.partial_cmp(b2).unwrap());
        for n in angles.iter().take(40) {
            println!("{:?}", n);
        }
        format!("{:?}", laser)
    }
}

pub fn get_day() -> Box<dyn AoCDay> {
    Box::new(Day10)
}
