use std::collections::VecDeque;
use regex::Regex;

pub fn solve(input: &str) {
    println!("Day {}.", file!().chars().filter(|c| c.is_digit(10)).collect::<String>());
    println!("Part 1: {}.", part_1::solve(&input, "abcdefgh"));
    println!("Part 2: {}.", part_2::solve(&input, "fbgdceah"));
    println!();
}

fn process(input: &str, password: &str, unscramble: bool) -> String {
    let re_swap_position = Regex::new(r"^swap position (?P<position_a>\d+) with position (?P<position_b>\d+)$").unwrap();
    let re_swap_letter = Regex::new(r"^swap letter (?P<letter_a>[a-z]) with letter (?P<letter_b>[a-z])$").unwrap();
    let re_rotate = Regex::new(r"^rotate (?P<direction>left|right) (?P<amount_steps>\d+) steps?$").unwrap();
    let re_rotate_based_on_position = Regex::new(r"^rotate based on position of letter (?P<letter>[a-z]+)$").unwrap();
    let re_reverse_positions = Regex::new(r"^reverse positions (?P<position_a>\d+) through (?P<position_b>\d+)$").unwrap();
    let re_move_position = Regex::new(r"^move position (?P<position_a>\d+) to position (?P<position_b>\d+)$").unwrap();
    let mut password_vector: VecDeque<char> = password.chars().collect();

    let lines: Vec<&str> = if unscramble {
        input.lines().rev().collect()
    } else {
        input.lines().collect()
    };

    for line in lines {
        if let Some(captures) = re_swap_position.captures(&line) {
            let position_a = captures.name("position_a").unwrap().as_str().parse().unwrap();
            let position_b = captures.name("position_b").unwrap().as_str().parse().unwrap();
            password_vector.swap(position_a, position_b);
        } else if let Some(captures) = re_swap_letter.captures(&line) {
            let letter_a = captures.name("letter_a").unwrap().as_str().chars().next().unwrap();
            let letter_b = captures.name("letter_b").unwrap().as_str().chars().next().unwrap();

            for index in 0..password_vector.len() {
                if password_vector[index] == letter_a {
                    password_vector[index] = letter_b;
                } else if password_vector[index] == letter_b {
                    password_vector[index] = letter_a;
                }
            }
        } else if let Some(captures) = re_rotate.captures(&line) {
            let direction = captures.name("direction").unwrap().as_str();
            let amount_steps = captures.name("amount_steps").unwrap().as_str().parse().unwrap();

            for _ in 0..amount_steps {
                match direction {
                    "left" => if unscramble {
                        rotate_right(&mut password_vector)
                    } else {
                        rotate_left(&mut password_vector)
                    },
                    "right" => if unscramble {
                        rotate_left(&mut password_vector)
                    } else {
                        rotate_right(&mut password_vector)
                    },
                    _ => panic!(),
                }
            }
        } else if let Some(captures) = re_rotate_based_on_position.captures(&line) {
            let letter = captures.name("letter").unwrap().as_str().chars().next().unwrap();
            let position = password_vector.iter().position(|&a_letter| a_letter == letter).unwrap();

            let amount_steps = if unscramble {
                match position {
                    0 => 1,
                    1 => 1,
                    2 => 6,
                    3 => 2,
                    4 => 7,
                    5 => 3,
                    6 => 0,
                    7 => 4,
                    _ => panic!(),
                }
            } else {
                if position >= 4 {
                    position + 2
                } else {
                    position + 1
                }
            };

            for _ in 0..amount_steps {
                if unscramble {
                    rotate_left(&mut password_vector);
                } else {
                    rotate_right(&mut password_vector);
                }
            }
        } else if let Some(captures) = re_reverse_positions.captures(&line) {
            let position_a: usize = captures.name("position_a").unwrap().as_str().parse().unwrap();
            let position_b: usize = captures.name("position_b").unwrap().as_str().parse().unwrap();
            let slice: Vec<char> = password_vector.drain(position_a..=position_b).collect();

            for character in slice {
                password_vector.insert(position_a, character);
            }
        } else if let Some(captures) = re_move_position.captures(&line) {
            let position_a: usize = captures.name("position_a").unwrap().as_str().parse().unwrap();
            let position_b: usize = captures.name("position_b").unwrap().as_str().parse().unwrap();
            let letter = password_vector.remove(if unscramble { position_b } else { position_a }).unwrap();
            password_vector.insert(if unscramble { position_a } else { position_b }, letter);
        } else {
            panic!();
        }
    }

    password_vector.iter().collect()
}

fn rotate_left(deque: &mut VecDeque<char>) {
    if let Some(popped_front) = deque.pop_front() {
        deque.push_back(popped_front);
    }
}

fn rotate_right(deque: &mut VecDeque<char>) {
    if let Some(popped_back) = deque.pop_back() {
        deque.push_front(popped_back);
    }
}

mod part_1 {
    use crate::day_21::process;

    pub fn solve(input: &str, password: &str) -> String {
        process(&input, password, false)
    }

    #[cfg(test)]
    #[test]
    fn test_1() {
        let input = "swap position 4 with position 0
swap letter d with letter b
reverse positions 0 through 4
rotate left 1 step
move position 1 to position 4
move position 3 to position 0
rotate based on position of letter b
rotate based on position of letter d";

        assert_eq!(solve(&input, "abcde"), "decab");
    }
}

mod part_2 {
    use crate::day_21::process;

    pub fn solve(input: &str, password: &str) -> String {
        process(&input, password, true)
    }
}
