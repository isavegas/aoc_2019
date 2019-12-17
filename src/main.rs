pub fn main() {
    let args = std::env::args().collect::<Vec<String>>();
    let mut selected_day: Option<usize> = None;
    let mut selected_part: Option<usize> = None;
    if args.len() > 1 {
        selected_day = Some(
            args[1]
                .parse()
                .expect("Unable to parse parameter as number"),
        );
    }
    if args.len() > 2 {
        selected_part = Some(
            args[2]
                .parse()
                .expect("Unable to parse parameter as number"),
        );
    }

    let days = aoc_2019::get_days();

    if let Some(s) = selected_day {
        let d = days
            .iter()
            .find(|d| d.day() == s)
            .expect("Selected day not found");
        let n = format!("{:02}", d.day());
        if let Some(p) = selected_part {
            match p {
                1 => println!("Day {}, Part 1: {}", n, d.part1()),
                2 => println!("Day {}, Part 2: {}", n, d.part2()),
                _ => println!("Selected part not found"),
            }
        } else {
            println!("Day {}, Part 1: {}", n, d.part1());
            println!("Day {}, Part 2: {}", n, d.part2());
        }
    } else {
    }
}
