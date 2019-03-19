pub fn solve(input: &str) {
    println!("Day {}.", file!().chars().filter(|c| c.is_digit(10)).collect::<String>());
    println!("Part 1: {}.", part_1::solve(&input));
    println!();
}

fn decode_input(input: &str) -> Vec<Vec<char>> {
    input.lines().map(|line| line.chars().collect()).collect()
}

#[derive(Debug)]
struct State {
    position: Position,
    amount_steps_taken: usize,
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
struct Position {
    x: usize,
    y: usize,
}

mod part_1 {
    use crate::day_24::{Position, State, decode_input};
    use std::collections::{HashSet, VecDeque};

    pub fn solve(input: &str) -> usize {
        let grid = decode_input(&input);
        let distances = get_distances_between_points_of_interest(&grid);
        get_shortest_path_size(&distances, Vec::new(), 0, 0).unwrap()
    }

    fn get_shortest_path_size(distances: &Vec<Vec<usize>>, mut path: Vec<usize>, path_size: usize, index: usize) -> Option<usize> {
        let mut shortest_path_size = None;
        path.push(index);

        if path.len() == distances.len() {
            return Some((1..path.len()).fold(0,|total, i| total + distances[path[i - 1]][path[i]]));
        }

        for index_other in 0..distances.len() {
            if index == index_other || path.contains(&index_other) {
                continue;
            }

            if let Some(path_size) = get_shortest_path_size(&distances, path.clone(), path_size + distances[index][index_other], index_other) {
                if shortest_path_size.is_none() || path_size < shortest_path_size.unwrap() {
                    shortest_path_size = Some(path_size);
                }
            }
        }

        shortest_path_size
    }

    fn get_distances_between_points_of_interest(grid: &Vec<Vec<char>>) -> Vec<Vec<usize>> {
        let points_of_interest_positions = get_points_of_interest_positions(&grid);
        let mut distances = vec![vec![std::usize::MAX; points_of_interest_positions.len()]; points_of_interest_positions.len()];

        for point_of_interest_a in 0..points_of_interest_positions.len() {
            for point_of_interest_b in 0..points_of_interest_positions.len() {
                distances[point_of_interest_a][point_of_interest_b] = get_distance(&grid, &points_of_interest_positions[point_of_interest_a], &points_of_interest_positions[point_of_interest_b]);
            }
        }

        distances
    }

    fn get_points_of_interest_positions(grid: &Vec<Vec<char>>) -> Vec<Position> {
        let mut positions = vec![None; get_amount_points_of_interest(&grid)];

        for (index_row, row) in grid.iter().enumerate() {
            for (index_column, &column) in row.iter().enumerate() {
                if let Some(digit) = column.to_digit(10) {
                    positions.insert(digit as usize, Some(Position {
                        x: index_column,
                        y: index_row,
                    }));
                }
            }
        }

        positions.into_iter().filter_map(|position| position).collect()
    }

    fn get_amount_points_of_interest(grid: &Vec<Vec<char>>) -> usize {
        grid.iter().flatten().filter(|tile| tile.is_digit(10)).count()
    }

    fn get_distance(grid: &Vec<Vec<char>>, from: &Position, to: &Position) -> usize {
        if from == to {
            return 0;
        }

        let mut queue = VecDeque::new();
        let mut visited_positions = HashSet::new();

        let initial_state = State {
            position: from.clone(),
            amount_steps_taken: 0,
        };

        queue.push_back(initial_state);

        while let Some(state) = queue.pop_front() {
            if &state.position == to {
                return state.amount_steps_taken;
            }

            visited_positions.insert(state.position.clone());
            try_add_new_position(&grid, &mut queue, &visited_positions, &state, 0, 1);
            try_add_new_position(&grid, &mut queue, &visited_positions, &state, 0, -1);
            try_add_new_position(&grid, &mut queue, &visited_positions, &state, 1, 0);
            try_add_new_position(&grid, &mut queue, &visited_positions, &state, -1, 0);
        }

        panic!();
    }

    fn try_add_new_position(grid: &Vec<Vec<char>>,
                            queue: &mut VecDeque<State>,
                            visited_positions: &HashSet<Position>,
                            state: &State,
                            offset_x: isize,
                            offset_y: isize) {
        let new_x = (state.position.x as isize + offset_x) as usize;
        let new_y = (state.position.y as isize + offset_y) as usize;

        if grid[new_y][new_x] == '#' {
            return;
        }

        let new_position = Position {
            x: new_x,
            y: new_y,
        };

        if visited_positions.contains(&new_position) {
            return;
        }

        if queue.iter().any(|state| state.position == new_position) {
            return;
        }

        let new_state = State {
            position: new_position,
            amount_steps_taken: state.amount_steps_taken + 1,
        };

        queue.push_back(new_state);
    }

    #[cfg(test)]
    #[test]
    fn test_1() {
        let input = "###########
#0.1.....2#
#.#######.#
#4.......3#
###########";

        assert_eq!(solve(&input), 14);
    }
}
