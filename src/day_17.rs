pub fn solve(input: &str) {
    println!("Day {}.", file!().chars().filter(|c| c.is_digit(10)).collect::<String>());
    println!("Part 1: {}.", part_1::solve(&input));
    println!();
}

#[derive(Clone, Debug)]
struct State {
    hash: String,
    steps_so_far: String,
    x: usize,
    y: usize,
}

mod part_1 {
    use std::collections::VecDeque;
    use crate::day_17::State;
    use crypto::md5::Md5;
    use crypto::digest::Digest;

    pub fn solve(input: &str) -> String {
        let mut md5_hasher = Md5::new();
        let hash = get_hash(&mut md5_hasher, &input);

        let current_state = State {
            hash,
            steps_so_far: String::new(),
            x: 1,
            y: 1,
        };

        let mut queue = VecDeque::new();
        queue.push_back(current_state);

        while let Some(state) = queue.pop_front() {
            if state.x == 4 && state.y == 4 {
                return state.steps_so_far;
            }

            let first_four_characters: Vec<char> = state.hash.chars().take(4).collect();
            try_add_new_state(&state, 'U', state.x, state.y - 1, first_four_characters[0], &input, &mut queue, &mut md5_hasher);
            try_add_new_state(&state, 'D', state.x, state.y + 1, first_four_characters[1], &input, &mut queue, &mut md5_hasher);
            try_add_new_state(&state, 'L', state.x - 1, state.y, first_four_characters[2], &input, &mut queue, &mut md5_hasher);
            try_add_new_state(&state, 'R', state.x + 1, state.y, first_four_characters[3], &input, &mut queue, &mut md5_hasher);
        }

        panic!();
    }

    fn try_add_new_state(current_state: &State,
                         direction: char,
                         new_x: usize,
                         new_y: usize,
                         character: char,
                         input: &str,
                         queue: &mut VecDeque<State>,
                         mut md5_hasher: &mut Md5) {
        if new_x >= 1 && new_x <= 4
                && new_y >= 1 && new_y <= 4
                && character >= 'b' && character <= 'f' {
            let new_input = format!("{}{}{}", input, current_state.steps_so_far, direction);
            let new_hash = get_hash(&mut md5_hasher, &new_input);
            let new_steps_so_far = format!("{}{}", current_state.steps_so_far, direction);

            queue.push_back(State {
                hash: new_hash,
                steps_so_far: new_steps_so_far,
                x: new_x,
                y: new_y,
            });
        }
    }

    fn get_hash(md5_hasher: &mut Md5, input: &str) -> String {
        md5_hasher.input_str(input);
        let hash = md5_hasher.result_str();
        md5_hasher.reset();
        hash
    }

    #[cfg(test)]
    #[test]
    #[should_panic]
    fn test_1() {
        solve("hijkl");
    }

    #[test]
    fn test_2() {
        assert_eq!(solve("ihgpwlah"), "DDRRRD");
    }

    #[test]
    fn test_3() {
        assert_eq!(solve("kglvqrro"), "DDUDRLRRUDRD");
    }

    #[test]
    fn test_4() {
        assert_eq!(solve("ulqzkmiv"), "DRURDRUDDLLDLUURRDULRLDUUDDDRR");
    }
}
