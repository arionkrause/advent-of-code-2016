pub fn solve(input: &str) {
    println!("Day {}.", file!().chars().filter(|c| c.is_digit(10)).collect::<String>());
    println!("Part 1: {}.", part_1::solve(&input, "abcdefgh"));
    println!();
}

mod part_1 {
    use regex::Regex;
    use std::collections::VecDeque;

    pub fn solve(input: &str, password: &str) -> String {
        let re_swap_position = Regex::new(r"^swap position (?P<position_a>\d+) with position (?P<position_b>\d+)$").unwrap();
        let re_swap_letter = Regex::new(r"^swap letter (?P<letter_a>[a-z]) with letter (?P<letter_b>[a-z])$").unwrap();
        let re_rotate = Regex::new(r"^rotate (?P<direction>left|right) (?P<amount_steps>\d+) steps?$").unwrap();
        let re_rotate_based_on_position = Regex::new(r"^rotate based on position of letter (?P<letter>[a-z]+)$").unwrap();
        let re_reverse_positions = Regex::new(r"^reverse positions (?P<position_a>\d+) through (?P<position_b>\d+)$").unwrap();
        let re_move_position = Regex::new(r"^move position (?P<position_a>\d+) to position (?P<position_b>\d+)$").unwrap();
        let mut scrambled_password: VecDeque<char> = password.chars().collect();

        for line in input.lines() {
            if let Some(captures) = re_swap_position.captures(&line) {
                let position_a = captures.name("position_a").unwrap().as_str().parse().unwrap();
                let position_b = captures.name("position_b").unwrap().as_str().parse().unwrap();
                scrambled_password.swap(position_a, position_b);
                continue;
            }

            if let Some(captures) = re_swap_letter.captures(&line) {
                let letter_a = captures.name("letter_a").unwrap().as_str().chars().next().unwrap();
                let letter_b = captures.name("letter_b").unwrap().as_str().chars().next().unwrap();

                for index in 0..scrambled_password.len() {
                    if scrambled_password[index] == letter_a {
                        scrambled_password[index] = letter_b;
                    } else if scrambled_password[index] == letter_b {
                        scrambled_password[index] = letter_a;
                    }
                }

                continue;
            }

            if let Some(captures) = re_rotate.captures(&line) {
                let direction = captures.name("direction").unwrap().as_str();
                let amount_steps = captures.name("amount_steps").unwrap().as_str().parse().unwrap();

                for _ in 0..amount_steps {
                    match direction {
                        "left" => rotate_left(&mut scrambled_password),
                        "right" => rotate_right(&mut scrambled_password),
                        _ => panic!(),
                    }
                }

                continue;
            }

            if let Some(captures) = re_rotate_based_on_position.captures(&line) {
                let letter = captures.name("letter").unwrap().as_str().chars().next().unwrap();
                let mut amount_steps = scrambled_password.iter().position(|&a_letter| a_letter == letter).unwrap();

                if amount_steps >= 4 {
                    amount_steps += 1;
                }

                for _ in 0..=amount_steps {
                    rotate_right(&mut scrambled_password);
                }

                continue;
            }

            if let Some(captures) = re_reverse_positions.captures(&line) {
                let position_a: usize = captures.name("position_a").unwrap().as_str().parse().unwrap();
                let position_b: usize = captures.name("position_b").unwrap().as_str().parse().unwrap();
                let slice: Vec<char> = scrambled_password.drain(position_a..=position_b).collect();

                for character in slice {
                    scrambled_password.insert(position_a, character);
                }

                continue;
            }

            if let Some(captures) = re_move_position.captures(&line) {
                let position_a: usize = captures.name("position_a").unwrap().as_str().parse().unwrap();
                let position_b: usize = captures.name("position_b").unwrap().as_str().parse().unwrap();
                let letter = scrambled_password.remove(position_a).unwrap();
                scrambled_password.insert(position_b, letter);
                continue;
            }

            panic!();
        }

        scrambled_password.iter().collect()
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
