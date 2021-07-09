mod day_01 {
    use aoc_2019::get_days;

    #[test]
    fn part1() {
        let days = get_days();
        let day = days.iter().nth(0).expect("Err");
        assert_eq!(
            day.expected().0.expect("Not provided"),
            day.part1().expect("Error")
        );
    }
    #[test]
    fn part2() {
        let days = get_days();
        let day = days.iter().nth(0).expect("Err");
        assert_eq!(
            day.expected().1.expect("Not provided"),
            day.part2().expect("Error")
        );
    }
}
