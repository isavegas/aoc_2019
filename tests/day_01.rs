use aoc_2019::day::day_01::get_day;

#[test]
pub fn part1_1() {
    assert_eq!(
        get_day()
            .part1("12")
            .expect("Error"),
        "2".to_string()
    );
}

#[test]
pub fn part1_2() {
    assert_eq!(
        get_day()
            .part1("14")
            .expect("Error"),
        "2".to_string()
    );
}

#[test]
pub fn part1_3() {
    assert_eq!(
        get_day()
            .part1("1969")
            .expect("Error"),
        "654".to_string()
    );
}

#[test]
pub fn part1_4() {
    assert_eq!(
        get_day()
            .part1("100756")
            .expect("Error"),
        "33583".to_string()
    );
}

#[test]
#[ignore]
pub fn part2_1() {
    assert_eq!(
        get_day()
            .part2("abc")
            .expect("Error"),
        "241861950".to_string()
    );
}
