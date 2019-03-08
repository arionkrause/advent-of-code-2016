pub fn solve(input: &str) {
    println!("Day {}.", file!().chars().filter(|c| c.is_digit(10)).collect::<String>());
    println!("Part 1: {}.", part_1::solve(&input));
    println!();
}

mod part_1 {
    use crypto::md5::Md5;
    use crypto::digest::Digest;
    use std::collections::VecDeque;

    pub fn solve(input: &str) -> usize {
        let mut amount_found_keys = 0;
        let mut number = 0;
        let mut next_number = 0;
        let mut hashes = VecDeque::with_capacity(1001);
        let mut md5_hasher = Md5::new();
        let salt = input.as_bytes();

        loop {
            while hashes.len() < 1001 {
                md5_hasher.input(salt);
                md5_hasher.input(next_number.to_string().as_bytes());
                hashes.push_back(md5_hasher.result_str());
                md5_hasher.reset();
                next_number += 1;
            }

            let hash = hashes.pop_front().unwrap();
            let chars = hash.chars().collect::<Vec<char>>();

            'hash: for index in 0..chars.len() - 2 {
                if chars[index] != chars[index + 1]
                        || chars[index] != chars[index + 2] {
                    continue;
                }

                for following_hash in hashes.iter() {
                    let following_hash_chars = following_hash.chars().collect::<Vec<char>>();

                    for following_hash_char_index in 0..following_hash_chars.len() - 4 {
                        if chars[index] != following_hash_chars[following_hash_char_index]
                                || following_hash_chars[following_hash_char_index] != following_hash_chars[following_hash_char_index + 1]
                                || following_hash_chars[following_hash_char_index] != following_hash_chars[following_hash_char_index + 2]
                                || following_hash_chars[following_hash_char_index] != following_hash_chars[following_hash_char_index + 3]
                                || following_hash_chars[following_hash_char_index] != following_hash_chars[following_hash_char_index + 4] {
                            continue;
                        }

                        amount_found_keys += 1;

                        if amount_found_keys == 64 {
                            return number;
                        }

                        break 'hash;
                    }
                }

                break 'hash;
            }

            md5_hasher.reset();
            number += 1;
        }
    }

    #[cfg(test)]
    #[test]
    fn test_1() {
        assert_eq!(solve("abc"), 22728);
    }
}
