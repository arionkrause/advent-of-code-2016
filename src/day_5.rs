pub fn solve(input: &str) {
    println!("Day {}.", file!().chars().filter(|c| c.is_digit(10)).collect::<String>());
    println!("Part 1: {}.", part_1::solve(&input));
    println!("Part 2: {}.", part_2::solve(&input));
    println!();
}

mod part_1 {
    use crypto::digest::Digest;
    use crypto::md5::Md5;

    pub fn solve(input: &str) -> String {
        let mut password = String::new();
        let mut index = 0;
        let mut md5_hasher = Md5::new();
        let key = input.as_bytes();

        loop {
            md5_hasher.input(key);
            md5_hasher.input(index.to_string().as_bytes());
            let mut hash = [0; 16];
            md5_hasher.result(&mut hash);

            if hash[0] as i32 == 0 && hash[1] as i32 == 0 && (hash[2] >> 4) as i32 == 0 {
                password.push_str(&format!("{:x}", hash[2]));

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

mod part_2 {
    use crypto::digest::Digest;
    use crypto::md5::Md5;

    pub fn solve(input: &str) -> String {
        let mut password = String::from("________");
        let mut index = 0;
        let mut md5_hasher = Md5::new();
        let key = input.as_bytes();

        loop {
            md5_hasher.input(key);
            md5_hasher.input(index.to_string().as_bytes());
            let mut hash = [0; 16];
            md5_hasher.result(&mut hash);

            if hash[0] as i32 == 0 && hash[1] as i32 == 0 && (hash[2] >> 4) as i32 == 0 {
                let position_as_character = format!("{:x}", hash[2]).chars().next().unwrap();

                match position_as_character.to_digit(10) {
                    Some(position) => {
                        if position < 8 && password.chars().nth(position as usize).unwrap() == '_' {
                            let character = format!("{:x}", hash[3] >> 4).chars().next().unwrap();
                            password.replace_range(position as usize..position as usize + 1, &character.to_string());

                            if !password.contains('_') {
                                return password;
                            }
                        }
                    }
                    None => {}
                }
            }

            md5_hasher.reset();
            index += 1;
        }
    }

    #[cfg(test)]
    #[test]
    fn test_1() {
        assert_eq!(solve("abc"), "05ace8e3");
    }
}
