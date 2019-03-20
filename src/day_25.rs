pub fn solve(input: &str) {
    println!("Day {}.", file!().chars().filter(|c| c.is_digit(10)).collect::<String>());
    println!("Part 1: {}.", part_1::solve(&input));
    println!();
}

mod part_1 {
    pub fn solve(input: &str) -> u16 {
        let value_c: u16 = input.lines().nth(1).unwrap().split_whitespace().nth(1).unwrap().parse().unwrap();
        let value_b: u16 = input.lines().nth(2).unwrap().split_whitespace().nth(1).unwrap().parse().unwrap();
        let product = value_c * value_b;
        let mut number = 0;

        loop {
            let result = product + number;
            let mut result_as_reversed_binary_vector: Vec<u8> = format!("{:b}", result).chars().map(|character| character.to_digit(10).unwrap() as u8).collect();
            result_as_reversed_binary_vector.reverse();

            if result_as_reversed_binary_vector.iter().enumerate().all(|(index, digit)| (index % 2) as u8 == *digit) {
                return number;
            }

            number += 1;
        }
    }
}
