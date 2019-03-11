use crypto::md5::Md5;
use crypto::digest::Digest;
use rayon::prelude::*;
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
    let batch_size = 10000;
    let mut hashes = VecDeque::with_capacity(batch_size);
    let mut fives = HashMap::new();
    let mut amount_batches_processed = 0;

    loop {
        if hashes.is_empty() || hashes.len() == 1001 {
            load_next_batch(&input, &mut hashes, &mut fives, batch_size, amount_batches_processed, amount_additional_hashings);
            amount_batches_processed += 1;
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

        number += 1;
    }
}

fn load_next_batch(salt: &str,
                   hashes: &mut VecDeque<String>,
                   fives: &mut HashMap<char, Vec<usize>>,
                   batch_size: usize,
                   amount_batches_processed: usize,
                   amount_additional_hashings: usize) -> usize {
    let first_of_next_numbers = amount_batches_processed * batch_size;
    let next_numbers = (first_of_next_numbers..(amount_batches_processed + 1) * batch_size).collect::<Vec<usize>>();
    let amount_cpus = num_cpus::get();
    let mut chunks_numbers = Vec::new();
    let batch_size = next_numbers.len();

    for i in 0..amount_cpus {
        if i == amount_cpus - 1 && batch_size % amount_cpus != 0 {
            // If batch size is not divisible by the amount of CPUs, the last chunk gets all the remaining items
            chunks_numbers.push((first_of_next_numbers + i * (batch_size / amount_cpus), first_of_next_numbers + batch_size));
        } else {
            chunks_numbers.push((first_of_next_numbers + i * (batch_size / amount_cpus), first_of_next_numbers + (i + 1) * (batch_size / amount_cpus)));
        }
    }

    let new_values: Vec<(usize, String, Option<(char, usize)>)> = chunks_numbers.par_iter().map(|(chunk_begin_number, chunk_end_number)| {
        let mut hashes = Vec::new();
        let mut md5_hasher = Md5::new();

        for a_number in *chunk_begin_number..*chunk_end_number {
            let mut hash = format!("{}{}", salt, a_number);

            for _ in 0..=amount_additional_hashings {
                md5_hasher.input_str(&hash);
                hash = md5_hasher.result_str();
                md5_hasher.reset();
            }

            let chars = hash.chars().collect::<Vec<char>>();
            let mut fives = None;

            for index in 0..chars.len() - 4 {
                if chars[index] != chars[index + 1]
                        || chars[index] != chars[index + 2]
                        || chars[index] != chars[index + 3]
                        || chars[index] != chars[index + 4] {
                    continue;
                }

                fives = Some((chars[index], a_number));
                break;
            }

            hashes.push((a_number, hash, fives));
            md5_hasher.reset();
        }

        hashes
    }).flatten()
            .collect();

    for (_, new_hash, new_fives) in new_values {
        hashes.push_back(new_hash);

        if let Some((new_character, new_fives)) = new_fives {
            fives.entry(new_character)
                    .and_modify(|numbers: &mut Vec<usize>| numbers.push(new_fives))
                    .or_insert(vec![new_fives]);
        }
    }

    *next_numbers.last().unwrap() + 1
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
