pub fn solve(input: &str) {
    println!("Day {}.", file!().chars().filter(|c| c.is_digit(10)).collect::<String>());
    println!("Part 1: {}.", part_1::solve(&input));
    println!("Part 2: {}.", part_2::solve(&input));
    println!();
}

mod part_1 {
    use std::collections::VecDeque;
    use crypto::digest::Digest;
    use crypto::md5::Md5;

    #[derive(Clone, Debug)]
    struct State {
        hash: String,
        path: String,
        x: usize,
        y: usize,
    }

    pub fn solve(input: &str) -> String {
        let mut md5_hasher = Md5::new();
        let hash = get_hash(&mut md5_hasher, &input);

        let current_state = State {
            hash,
            path: String::new(),
            x: 1,
            y: 1,
        };

        let mut queue = VecDeque::new();
        queue.push_back(current_state);

        while let Some(state) = queue.pop_front() {
            if state.x == 4 && state.y == 4 {
                return state.path;
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
            let new_input = format!("{}{}{}", input, current_state.path, direction);
            let new_hash = get_hash(&mut md5_hasher, &new_input);
            let new_path = format!("{}{}", current_state.path, direction);

            queue.push_back(State {
                hash: new_hash,
                path: new_path,
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

mod part_2 {
    use crypto::digest::Digest;
    use crypto::md5::Md5;

    #[derive(Clone, Debug)]
    struct State {
        hash: String,
        path: String,
        amount_steps_so_far: usize,
        x: usize,
        y: usize,
    }

    pub fn solve(input: &str) -> usize {
        let mut md5_hasher = Md5::new();
        let hash = get_hash(&mut md5_hasher, &input);
        let mut maximum_amount_steps = None;

        let current_state = State {
            hash,
            path: String::new(),
            amount_steps_so_far: 0,
            x: 1,
            y: 1,
        };

        let mut queue = Vec::new();
        queue.push(current_state);

        while let Some(state) = queue.pop() {
            if state.x == 4 && state.y == 4 {
                if maximum_amount_steps.is_none() || state.amount_steps_so_far > maximum_amount_steps.unwrap() {
                    maximum_amount_steps = Some(state.amount_steps_so_far);
                }

                continue;
            }

            let first_four_characters: Vec<char> = state.hash.chars().take(4).collect();
            try_add_new_state(&state, 'U', state.x, state.y - 1, first_four_characters[0], &input, &mut queue, &mut md5_hasher);
            try_add_new_state(&state, 'D', state.x, state.y + 1, first_four_characters[1], &input, &mut queue, &mut md5_hasher);
            try_add_new_state(&state, 'L', state.x - 1, state.y, first_four_characters[2], &input, &mut queue, &mut md5_hasher);
            try_add_new_state(&state, 'R', state.x + 1, state.y, first_four_characters[3], &input, &mut queue, &mut md5_hasher);
        }

        maximum_amount_steps.unwrap()
    }

    fn try_add_new_state(current_state: &State,
                         direction: char,
                         new_x: usize,
                         new_y: usize,
                         character: char,
                         input: &str,
                         queue: &mut Vec<State>,
                         mut md5_hasher: &mut Md5) {
        if new_x >= 1 && new_x <= 4
                && new_y >= 1 && new_y <= 4
                && character >= 'b' && character <= 'f' {
            let new_input = format!("{}{}{}", input, current_state.path, direction);
            let new_hash = get_hash(&mut md5_hasher, &new_input);
            let new_path = format!("{}{}", current_state.path, direction);

            queue.push(State {
                hash: new_hash,
                path: new_path,
                amount_steps_so_far: current_state.amount_steps_so_far + 1,
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
    fn test_1() {
        assert_eq!(solve("ihgpwlah"), 370);
    }

    #[test]
    fn test_2() {
        assert_eq!(solve("kglvqrro"), 492);
    }

    #[test]
    fn test_3() {
        assert_eq!(solve("ulqzkmiv"), 830);
    }
}
