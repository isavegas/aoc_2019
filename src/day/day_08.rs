use crate::AoCDay;
use lazy_static::lazy_static;

pub struct Day08;

type Num = u8;

const INPUT: &'static str = include_str!("../input/day_08.txt");

lazy_static! {
    static ref DATA: Vec<Num> = INPUT
        .trim()
        .chars()
        .map(|d| d.to_digit(10).unwrap() as u8)
        .collect();
}

impl AoCDay for Day08 {
    fn day(&self) -> usize {
        08
    }
    fn part1(&self) -> String {
        let data = &*DATA;
        let mut layers = vec![];
        let layer_size = 25 * 6;
        let mut o = 0;
        while o < data.len() - 1 {
            layers.push(&data[o..(o + layer_size)]);
            o += layer_size;
        }

        let (target_index, _) = layers
            .iter()
            .map(|l| bytecount::count(l, 0))
            .enumerate()
            .min_by_key(|(_, c)| *c)
            .unwrap();
        let layer = layers[target_index];

        format!(
            "{}",
            bytecount::count(layer, 1) * bytecount::count(layer, 2)
        )
    }
    fn part2(&self) -> String {
        let data = &*DATA;
        let mut layers = vec![];
        let width = 25;
        let height = 6;
        let layer_size = width * height;
        let mut o = 0;
        while o < data.len() - 1 {
            layers.push(&data[o..(o + layer_size)]);
            o += layer_size;
        }
        let block = std::char::from_u32(9608).expect("Invalid block character");
        let mut out: Vec<char> = Vec::with_capacity(layer_size);
        for i in 0..layer_size {
            for l in layers.iter() {
                match l[i] {
                    0 => {
                        out.push(' ');
                        break;
                    }
                    1 => {
                        out.push(block);
                        break;
                    }
                    2 => (),
                    _ => panic!("Invalid data!"),
                }
            }
        }
        for r in 0..height {
            let start = r * width;
            let end = start + width;
            println!("{}", out[start..end].iter().collect::<String>());
        }
        format!("Image generated")
    }
}

pub fn get_day() -> Box<dyn AoCDay> {
    Box::new(Day08)
}
