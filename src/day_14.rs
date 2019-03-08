use crypto::md5::Md5;
use crypto::digest::Digest;
use std::collections::{HashMap, VecDeque};

pub fn solve(input: &str) {
    println!("Day {}.", file!().chars().filter(|c| c.is_digit(10)).collect::<String>());
    println!("Part 1: {}.", part_1::solve(&input));
    println!("Part 2: {}.", part_2::solve(&input));
    println!();
}

fn get_index_64th_key(input: &str, amount_additional_hashings: usize) -> usize {
    let mut amount_found_keys = 0;
    let mut number = 0;
    let mut next_number = 0;
    let mut hashes = VecDeque::with_capacity(1001);
    let mut fives = HashMap::new();
    let mut md5_hasher = Md5::new();

    loop {
        while hashes.len() < 1001 {
            let mut buffer = format!("{}{}", input, next_number);

            for _ in 0..=amount_additional_hashings {
                md5_hasher.input_str(&buffer);
                buffer = md5_hasher.result_str();
                md5_hasher.reset();
            }

            let chars = buffer.chars().collect::<Vec<char>>();

            for index in 0..chars.len() - 4 {
                if chars[index] != chars[index + 1]
                        || chars[index] != chars[index + 2]
                        || chars[index] != chars[index + 3]
                        || chars[index] != chars[index + 4] {
                    continue;
                }

                fives.entry(chars[index])
                        .and_modify(|numbers: &mut Vec<usize>| numbers.push(next_number))
                        .or_insert(vec![next_number]);

                break;
            }

            hashes.push_back(buffer);
            next_number += 1;
        }

        let hash = hashes.pop_front().unwrap();
        let chars = hash.chars().collect::<Vec<char>>();

        'hash: for index in 0..chars.len() - 2 {
            if chars[index] != chars[index + 1]
                    || chars[index] != chars[index + 2] {
                continue;
            }

            match fives.get(&chars[index]) {
                Some(indices) => {
                    if indices.iter().any(|five_number| *five_number > number && *five_number - number <= 1000) {
                        amount_found_keys += 1;

                        if amount_found_keys == 64 {
                            return number;
                        }
                    }
                }
                None => {}
            }

            break 'hash;
        }

        md5_hasher.reset();
        number += 1;
    }
}

mod part_1 {
    use crate::day_14::get_index_64th_key;

    pub fn solve(input: &str) -> usize {
        get_index_64th_key(&input, 0)
    }

    #[cfg(test)]
    #[test]
    fn test_1() {
        assert_eq!(solve("abc"), 22728);
    }
}

mod part_2 {
    use crate::day_14::get_index_64th_key;

    pub fn solve(input: &str) -> usize {
        get_index_64th_key(&input, 2016)
    }

    #[cfg(test)]
    #[test]
    fn test_1() {
        assert_eq!(solve("abc"), 22551);
    }
}
