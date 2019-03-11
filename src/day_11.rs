use regex::Regex;
use std::collections::HashSet;
use std::collections::VecDeque;

pub fn solve(input: &str) {
    println!("Day {}.", file!().chars().filter(|c| c.is_digit(10)).collect::<String>());
    println!("Part 1: {}.", part_1::solve(&input));
    println!("Part 2: {}.", part_2::solve(&input));
    println!();
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
struct State {
    items: Vec<Item>,
    elevator_floor: usize,
    steps_taken: usize,
    combination: usize,
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
struct Item {
    item_type: usize,
    id: usize,
    floor: usize,
}

fn decode_input(input: &str) -> State {
    let re = Regex::new(r"(?P<item>[a-z]+)(-compatible)? (?P<item_type>microchip|generator)").unwrap();
    let mut items = Vec::new();

    for (index, line) in input.lines().enumerate() {
        for captures in re.captures_iter(&line) {
            items.push(Item {
                item_type: match captures.name("item_type").unwrap().as_str() {
                    "generator" => 0,
                    "microchip" => 1,
                    _ => panic!(),
                },
                id: captures.name("item").unwrap().as_str().to_string().chars().map(|character| character as usize).sum::<usize>(),
                floor: index + 1,
            });
        }
    }

    State {
        items: items.clone(),
        elevator_floor: 1,
        steps_taken: 0,
        combination: get_combination(&items, 1),
    }
}

fn get_minimum_amount_steps(state: State) -> usize {
    let mut queue = VecDeque::new();
    let mut seem_state_combinations = HashSet::new();
    queue.push_back(state);

    loop {
        let state = queue.pop_front().unwrap();

        if state.elevator_floor == 4
                // Using negated "any"+"different" instead of "all"+"equal" shortcircuits comparison
                && !state.items.iter().any(|item| item.floor != 4) {
            return state.steps_taken;
        }

        seem_state_combinations.insert(state.combination);

        for next_state in get_new_states(&state) {
            if !seem_state_combinations.contains(&next_state.combination)
                && !queue.iter().any(|state| state.combination == next_state.combination) {
                queue.push_back(next_state);
            }
        }
    }
}

fn get_new_states(state: &State) -> Vec<State> {
    let mut new_states = Vec::new();

    if state.elevator_floor > 1 {
        new_states.extend(get_new_states_move_items(&state, -1));
    }

    if state.elevator_floor < 4 {
        new_states.extend(get_new_states_move_items(&state, 1));
    }

    new_states
}

fn get_new_states_move_items(state: &State, floor_offset: isize) -> Vec<State> {
    let mut new_states = Vec::new();
    let new_elevator_floor = (state.elevator_floor as isize + floor_offset) as usize;

    state.items.iter().enumerate()
            .filter(|(_, item)| item.floor == state.elevator_floor)
            .for_each(|(index, item)| {
                let mut next_state_moving_1_item = state.clone().to_owned();
                next_state_moving_1_item.elevator_floor = new_elevator_floor;
                next_state_moving_1_item.items[index].floor = new_elevator_floor;
                next_state_moving_1_item.steps_taken += 1;
                let mut can_move_one_item = false;

                if is_state_valid(&next_state_moving_1_item) {
                    can_move_one_item = true;
                }

                let mut can_move_two_items = false;

                state.items.iter().enumerate()
                        .filter(|(_, other_item)| {
                            other_item.floor == state.elevator_floor
                                    && (other_item.item_type != item.item_type
                                    || other_item.id != item.id)
                        })
                        .for_each(|(other_index, _)| {
                            let mut next_state_moving_2_items = next_state_moving_1_item.clone().to_owned();
                            next_state_moving_2_items.items[other_index].floor = new_elevator_floor;

                            if is_state_valid(&next_state_moving_2_items) {
                                can_move_two_items = true;
                                next_state_moving_2_items.combination = get_combination(&next_state_moving_2_items.items, next_state_moving_2_items.elevator_floor);
                                new_states.push(next_state_moving_2_items);
                            }
                        });

                if (floor_offset == -1 && can_move_one_item) || (floor_offset == 1 && !can_move_two_items && can_move_one_item) {
                    next_state_moving_1_item.combination = get_combination(&next_state_moving_1_item.items, next_state_moving_1_item.elevator_floor);
                    new_states.push(next_state_moving_1_item);
                }
            });

    new_states
}

fn is_state_valid(state: &State) -> bool {
    state.items.iter().filter(|item| item.item_type == 1)
            .all(|item| {
                state.items.iter().find(|other_item| other_item.item_type == 0 && other_item.floor == item.floor && other_item.id != item.id).is_none()
                        || state.items.iter().find(|other_item| other_item.item_type == 0 && other_item.floor == item.floor && other_item.id == item.id).is_some()
            })
}

fn get_combination(items: &Vec<Item>, elevator_floor: usize) -> usize {
    let mut pairs = Vec::new();

    for (item_index, item) in items.iter().enumerate() {
        if item.item_type == 0 {
            for (other_item_index, other_item) in items.iter().enumerate() {
                if other_item.item_type == 1 && other_item.id == item.id {
                    pairs.push((item_index, other_item_index));
                    break;
                }
            }
        }
    }

    pairs.sort_by(|a, b| items[a.0].floor.cmp(&items[b.0].floor)
            .then(items[a.1].floor.cmp(&items[b.1].floor)));

    let mut state_combination = elevator_floor;
    let mut multiplier = 10;

    for (generator_index, corresponding_microchip_index) in pairs.iter() {
        state_combination += multiplier * items[*generator_index].floor;
        multiplier *= 10;
        state_combination += multiplier * items[*corresponding_microchip_index].floor;
        multiplier *= 10;
    }

    state_combination
}

mod part_1 {
    use crate::day_11::decode_input;
    use crate::day_11::get_minimum_amount_steps;

    pub fn solve(input: &str) -> usize {
        let state = decode_input(&input);
        get_minimum_amount_steps(state)
    }

    #[cfg(test)]
    #[test]
    fn test_1() {
        let input = "The first floor contains a hydrogen-compatible microchip and a lithium-compatible microchip.
The second floor contains a hydrogen generator.
The third floor contains a lithium generator.
The fourth floor contains nothing relevant.";

        assert_eq!(solve(&input), 11);
    }
}

mod part_2 {
    use crate::day_11::decode_input;
    use crate::day_11::get_minimum_amount_steps;
    use crate::day_11::Item;

    pub fn solve(input: &str) -> usize {
        let mut state = decode_input(&input);

        state.items.push(Item {
            item_type: 0,
            id: String::from("elerium").chars().map(|character| character as usize).sum::<usize>(),
            floor: 1,
        });

        state.items.push(Item {
            item_type: 1,
            id: String::from("elerium").chars().map(|character| character as usize).sum::<usize>(),
            floor: 1,
        });

        state.items.push(Item {
            item_type: 0,
            id: String::from("dilithium").chars().map(|character| character as usize).sum::<usize>(),
            floor: 1,
        });

        state.items.push(Item {
            item_type: 1,
            id: String::from("dilithium").chars().map(|character| character as usize).sum::<usize>(),
            floor: 1,
        });

        get_minimum_amount_steps(state)
    }
}
