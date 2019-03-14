pub fn solve(input: &str) {
    println!("Day {}.", file!().chars().filter(|c| c.is_digit(10)).collect::<String>());
    println!("Part 1: {}.", part_1::solve(&input));
    println!("Part 2: {}.", part_2::solve(&input));
    println!();
}

mod part_1 {
    pub fn solve(input: &str) -> usize {
        // This is a case of the "Josephus problem"
        // Numberphile video about it: https://www.youtube.com/watch?v=uCsD3ZGzMgE
        let input_as_number = input.parse::<usize>().unwrap();
        let mut number_as_binary = format!("{:b}", input_as_number);
        let first_digit_of_number_as_binary = number_as_binary.remove(0);
        number_as_binary.push(first_digit_of_number_as_binary);
        usize::from_str_radix(&number_as_binary, 2).unwrap()
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
