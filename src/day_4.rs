use regex::Regex;
use std::collections::HashMap;

pub fn solve(input: &str) {
    println!("Day {}.", file!().chars().filter(|c| c.is_digit(10)).collect::<String>());
    println!("Part 1: {}.", part_1::solve(&input));
    println!();
}

#[derive(Debug)]
struct Room {
    name: String,
    id: u16,
    checksum: String,
}

#[derive(Debug)]
struct LetterFrequency {
    letter: char,
    frequency: u8,
}

fn decode_input(input: &str) -> Vec<Room> {
    let re = Regex::new(r"^(?P<name>[a-z\-]+?)-(?P<id>\d+)\[(?P<checksum>[a-z]+)]$").unwrap();

    input.lines().map(|line| {
        let captures = re.captures(&line).unwrap();

        Room {
            name: captures.name("name").unwrap().as_str().to_string(),
            id: captures.name("id").unwrap().as_str().parse().unwrap(),
            checksum: captures.name("checksum").unwrap().as_str().to_string(),
        }
    }).collect()
}

fn get_letters_frequencies_descending(input: &str) -> Vec<LetterFrequency> {
    let mut letters_frequencies: HashMap<char, LetterFrequency> = HashMap::new();

    for letter in input.chars().filter(|character| character.is_alphabetic()) {
        letters_frequencies.entry(letter)
                .and_modify(|letter_frequency| letter_frequency.frequency += 1)
                .or_insert(LetterFrequency { letter, frequency: 1 });
    }

    let mut letters_frequencies_descending: Vec<LetterFrequency> = letters_frequencies.into_iter()
            .map(|(_, letter_frequency)| letter_frequency)
            .collect();

    letters_frequencies_descending.sort_by(|a, b| {
        b.frequency.cmp(&a.frequency)
                .then(a.letter.cmp(&b.letter))
    });

    letters_frequencies_descending
}

mod part_1 {
    use crate::day_4::decode_input;
    use crate::day_4::get_letters_frequencies_descending;

    pub fn solve(input: &str) -> u32 {
        decode_input(&input).iter().filter(|room| {
            let letters_frequencies = get_letters_frequencies_descending(&room.name);

            for (index, letter) in room.checksum.chars().enumerate() {
                if letter != letters_frequencies[index].letter {
                    return false;
                }
            }

            true
        })
                .map(|room| room.id as u32)
                .sum()
    }

    #[cfg(test)]
    #[test]
    fn test_1() {
        let input = "aaaaa-bbb-z-y-x-123[abxyz]
a-b-c-d-e-f-g-h-987[abcde]
not-a-real-room-404[oarel]
totally-real-room-200[decoy]";

        assert_eq!(solve(&input), 1514);
    }
}
