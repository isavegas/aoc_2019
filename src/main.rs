use aoc_core::AoCDay;

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

    if let Some(day) = selected_day {
        let day_impl = days
            .iter()
            .find(|d| d.day() == day)
            .expect("Selected day not found");

        run_day(day_impl, selected_part);
    } else {
        for day_impl in days.iter() {
            run_day(day_impl, None);
        }
    }
}

#[derive(Debug)]
enum TestStatus {
    Unknown,
    Failure,
    Success,
}
impl std::fmt::Display for TestStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Unknown => write!(f, "?"),
            Self::Failure => write!(f, "✗"),
            Self::Success => write!(f, "✓"),
        }
    }
}

// Clippy's suggestion for `&AoCDay` instead doesn't work. I need `Box<>` here
#[allow(clippy::borrowed_box)]
fn run_day(day_impl: &Box<dyn AoCDay>, part: Option<usize>) {
    match part {
        Some(p) => {
            let (expected, val) = match p {
                1 => (day_impl.expected().0, day_impl.part1()),
                2 => (day_impl.expected().1, day_impl.part2()),
                _ => unreachable!(),
            };

            let (status1, value) = check_status(expected, val);
            println!(
                "Day {:02}, Part {}: {} {}",
                day_impl.day(),
                p,
                status1,
                value
            );
        }
        None => {
            let (status1, value1) = check_status(day_impl.expected().0, day_impl.part1());
            let (status2, value2) = check_status(day_impl.expected().1, day_impl.part2());
            println!("Day {:02}, Part 1: {} {}", day_impl.day(), status1, value1);
            println!("        Part 2: {} {}", status2, value2);
        }
    }
}

fn check_status(
    expected: Option<&str>,
    value: Result<String, aoc_core::ErrorWrapper>,
) -> (TestStatus, String) {
    match value {
        Ok(val) => {
            let status = match expected {
                Some(v) => match v == val {
                    true => TestStatus::Success,
                    false => TestStatus::Failure,
                },
                None => TestStatus::Unknown,
            };
            (status, val)
        }
        Err(err) => (TestStatus::Failure, err.to_string()),
    }
}
