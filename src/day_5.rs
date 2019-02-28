use crypto::digest::Digest;
use crypto::md5::Md5;
use rayon::prelude::*;

pub fn solve(input: &str) {
    println!("Day {}.", file!().chars().filter(|c| c.is_digit(10)).collect::<String>());
    println!("Part 1: {}.", part_1::solve(&input));
    println!("Part 2: {}.", part_2::solve(&input));
    println!();
}

fn get_hashes_starting_with_five_zeros(key: &[u8], initial_index: usize, batch_size: usize) -> Vec<Vec<u8>> {
    let amount_cpus = num_cpus::get();
    let mut chunks_indices = Vec::new();

    for i in 0..amount_cpus {
        if i == amount_cpus - 1 && batch_size % amount_cpus != 0 {
            // If batch size is not divisible by the amount of CPUs, the last chunk gets all the remaining items
            chunks_indices.push((initial_index + i * (batch_size / amount_cpus), initial_index + batch_size));
        } else {
            chunks_indices.push((initial_index + i * (batch_size / amount_cpus), initial_index + (i + 1) * (batch_size / amount_cpus)));
        }
    }

    chunks_indices.par_iter().map(|(chunk_begin_index, chunk_end_index)| {
        let mut hashes = Vec::new();
        let mut md5_hasher = Md5::new();

        for index in *chunk_begin_index..*chunk_end_index {
            md5_hasher.input(key);
            md5_hasher.input(index.to_string().as_bytes());
            let mut hash = [0; 16];
            md5_hasher.result(&mut hash);

            if hash[0] as i32 == 0 && hash[1] as i32 == 0 && (hash[2] >> 4) as i32 == 0 {
                hashes.push(hash.to_vec());
            }

            md5_hasher.reset();
        }

        hashes
    }).flatten()
            .collect()
}

mod part_1 {
    use crate::day_5::get_hashes_starting_with_five_zeros;

    pub fn solve(input: &str) -> String {
        let mut password = String::new();
        let mut index = 0;
        let batch_size = 1_000_000;
        let key = input.as_bytes();

        loop {
            let hashes = get_hashes_starting_with_five_zeros(&key, index, batch_size);

            for hash in hashes {
                if hash[0] as i32 == 0 && hash[1] as i32 == 0 && (hash[2] >> 4) as i32 == 0 {
                    password.push_str(&format!("{:x}", hash[2]));

                    if password.len() == 8 {
                        return password;
                    }
                }
            }

            index += batch_size;
        }
    }

    #[cfg(test)]
    #[test]
    fn test_1() {
        assert_eq!(solve("abc"), "18f47a30");
    }
}

mod part_2 {
    use crate::day_5::get_hashes_starting_with_five_zeros;

    pub fn solve(input: &str) -> String {
        let mut password = String::from("________");
        let mut index = 0;
        let batch_size = 1_000_000;
        let key = input.as_bytes();

        loop {
            let hashes = get_hashes_starting_with_five_zeros(&key, index, batch_size);

            for hash in hashes {
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
            }

            index += batch_size;
        }
    }

    #[cfg(test)]
    #[test]
    fn test_1() {
        assert_eq!(solve("abc"), "05ace8e3");
    }
}
