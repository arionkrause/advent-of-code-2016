pub fn solve(input: &str) {
    println!("Day {}.", file!().chars().filter(|c| c.is_digit(10)).collect::<String>());
    println!("Part 1: {}.", part_1::solve(&input));
    println!();
}

mod part_1 {
    use crypto::digest::Digest;
    use crypto::md5::Md5;

    pub fn solve(input: &str) -> String {
        let mut password = String::new();
        let mut index = 0;
        let mut md5_hasher = Md5::new();

        loop {
            md5_hasher.input_str(&format!("{}{}", input, index));
            let result = md5_hasher.result_str();

            if result.starts_with("00000") {
                password.push(result.chars().nth(5).unwrap());

                if password.len() == 8 {
                    return password;
                }
            }

            md5_hasher.reset();
            index += 1;
        }
    }

    #[cfg(test)]
    #[test]
    fn test_1() {
        assert_eq!(solve("abc"), "18f47a30");
    }
}
