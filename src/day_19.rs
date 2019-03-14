pub fn solve(input: &str) {
    println!("Day {}.", file!().chars().filter(|c| c.is_digit(10)).collect::<String>());
    println!("Part 1: {}.", part_1::solve(&input));
    println!("Part 2: {}.", part_2::solve(&input));
    println!();
}

mod part_1 {
    pub fn solve(input: &str) -> usize {
        let input_as_number = input.parse::<usize>().unwrap();
        let highest_power_of_two_lower_than_input = 2usize.pow((input_as_number as f32).log2() as u32) as usize;
        1 + 2 * (input_as_number - highest_power_of_two_lower_than_input)
    }

    #[cfg(test)]
    #[test]
    fn test_1() {
        assert_eq!(solve("5"), 3);
    }
}

mod part_2 {
    pub fn solve(input: &str) -> usize {
        let input_as_number = input.parse::<usize>().unwrap();
        let highest_power_of_three_lower_than_input = 3usize.pow(((input_as_number - 1) as f32).log(3f32) as u32) as usize;

        input_as_number - highest_power_of_three_lower_than_input
            + ((input_as_number as isize - 2 * highest_power_of_three_lower_than_input as isize).max(0)) as usize
    }

    #[cfg(test)]
    #[test]
    fn test_1() {
        assert_eq!(solve("5"), 2);
    }
}
