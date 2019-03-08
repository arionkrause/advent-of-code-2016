use std::collections::{HashMap, VecDeque};

pub fn solve(input: &str) {
    println!("Day {}.", file!().chars().filter(|c| c.is_digit(10)).collect::<String>());
    println!("Part 1: {}.", part_1::solve(&input, 31, 39, false));
    println!("Part 2: {}.", part_2::solve(&input));
    println!();
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
struct Position {
    x: usize,
    y: usize,
}

fn get_grid(office_designer_favorite_number: usize, maximum_x: usize, maximum_y: usize) -> Vec<Vec<char>> {
    let mut grid = Vec::new();

    for y in 0..=maximum_y {
        let mut row = Vec::new();

        for x in 0..=maximum_x {
            row.push(get_tile(office_designer_favorite_number, x, y));
        }

        grid.push(row);
    }

    grid
}

fn get_tile(office_designer_favorite_number: usize, x: usize, y: usize) -> char {
    let mut result = (x * x) + (3 * x) + (2 * x * y) + (y) + (y * y);
    result += office_designer_favorite_number;

    if result.count_ones() % 2 == 0 {
        '.'
    } else {
        '#'
    }
}

fn add_new_positions(mut grid: &mut Vec<Vec<char>>,
                     position: &Position,
                     amount_steps: usize,
                     queue: &mut VecDeque<(Position, usize)>,
                     visited_positions: &Vec<Position>,
                     previous_positions: &mut HashMap<Position, (Position, usize)>,
                     office_designer_favorite_number: usize) {
    let mut new_positions = Vec::new();

    match try_get_new_position(&mut grid, &position, 0, -1, office_designer_favorite_number) {
        Some(position) => new_positions.push(position),
        None => {}
    }

    match try_get_new_position(&mut grid, &position, 1, 0, office_designer_favorite_number) {
        Some(position) => new_positions.push(position),
        None => {}
    }

    match try_get_new_position(&mut grid, &position, 0, 1, office_designer_favorite_number) {
        Some(position) => new_positions.push(position),
        None => {}
    }

    match try_get_new_position(&mut grid, &position, -1, 0, office_designer_favorite_number) {
        Some(position) => new_positions.push(position),
        None => {}
    }

    for new_position in new_positions {
        if visited_positions.contains(&new_position)
                || queue.iter().find(|(position, _)| position == &new_position).is_some() {
            continue;
        }

        match previous_positions.get(&new_position) {
            Some((_, previous_position_amount_steps)) => {
                if amount_steps < *previous_position_amount_steps {
                    previous_positions.insert(new_position.clone(), (position.clone(), amount_steps));
                }
            }
            None => {
                previous_positions.insert(new_position.clone(), (position.clone(), amount_steps));
            }
        }

        queue.push_back((new_position, amount_steps + 1));
    }
}

fn try_get_new_position(grid: &mut Vec<Vec<char>>,
                         position: &Position,
                         offset_x: isize,
                         offset_y: isize,
                         office_designer_favorite_number: usize) -> Option<Position> {
    let new_x = position.x as isize + offset_x;

    if new_x < 0 {
        return None;
    }

    let new_y = position.y as isize + offset_y;

    if new_y < 0 {
        return None;
    }

    let new_x = new_x as usize;
    let new_y = new_y as usize;

    if new_x == grid[0].len() {
        for (index_row, row) in grid.iter_mut().enumerate() {
            row.push(get_tile(office_designer_favorite_number, new_x, index_row));
        }
    }

    if new_y == grid.len() {
        let mut row = Vec::new();

        for x in 0..grid[0].len() {
            row.push(get_tile(office_designer_favorite_number, x, new_y));
        }

        grid.push(row);
    }

    if grid[new_y as usize][new_x as usize] == '#' {
        return None;
    }

    Some(Position {
        x: new_x,
        y: new_y,
    })
}

fn print_path(grid: &Vec<Vec<char>>, start: &Position, target: &Position, previous_positions: &HashMap<Position, (Position, usize)>) {
    let mut visited_positions = Vec::new();
    let mut current_position = target;

    while let Some((previous_position, _)) = previous_positions.get(current_position) {
        visited_positions.push(current_position.clone());
        current_position = previous_position;
    }

    print_grid(&grid, &visited_positions, &start, &target);
}

fn print_grid(grid: &Vec<Vec<char>>, visited_positions: &Vec<Position>, start: &Position, target: &Position) {
    for (index_row, row) in grid.iter().enumerate() {
        for (index_tile, tile) in row.iter().enumerate() {
            let position = Position { x: index_tile, y: index_row };

            if &position == start {
                print!("o");
                continue;
            }

            if &position == target {
                print!("X");
                continue;
            }

            if visited_positions.contains(&position) {
                print!("O");
                continue;
            }

            print!("{}", tile);
        }

        println!();
    }
}

mod part_1 {
    use std::collections::{HashMap, VecDeque};
    use crate::day_13::{Position, add_new_positions, get_grid, print_grid, print_path};

    pub fn solve(input: &str, target_x: usize, target_y: usize, print_grid_and_path: bool) -> usize {
        let office_designer_favorite_number = input.parse().unwrap();
        let mut grid = get_grid(office_designer_favorite_number, target_x, target_y);
        let mut queue = VecDeque::new();
        let start = Position { x: 1, y: 1};
        let target = Position { x: target_x, y: target_y};
        queue.push_back((start.clone(), 0));
        let mut previous_positions = HashMap::new();
        let mut visited_positions = Vec::new();

        while let Some((position, amount_steps)) = queue.pop_front() {
            if position == target {
                if print_grid_and_path {
                    print_grid(&grid, &visited_positions, &start, &target);
                    println!();
                    print_path(&grid, &start, &target, &previous_positions);
                }

                return amount_steps;
            }

            visited_positions.push(position.clone());
            add_new_positions(&mut grid, &position, amount_steps, &mut queue, &visited_positions, &mut previous_positions, office_designer_favorite_number);
        }

        panic!()
    }

    #[cfg(test)]
    #[test]
    fn test_1() {
        assert_eq!(solve("10", 7, 4, true), 11);
    }
}

mod part_2 {
    use std::collections::{HashMap, VecDeque};
    use crate::day_13::{Position, add_new_positions, get_grid};

    pub fn solve(input: &str) -> usize {
        let office_designer_favorite_number = input.parse().unwrap();
        let mut grid = get_grid(office_designer_favorite_number, 2, 2);
        get_amount_reachable_positions(&mut grid, Position { x: 1, y: 1 }, office_designer_favorite_number)
    }

    fn get_amount_reachable_positions(mut grid: &mut Vec<Vec<char>>, start: Position, office_designer_favorite_number: usize) -> usize {
        let mut queue = VecDeque::new();
        queue.push_back((start.clone(), 0));
        let mut previous_positions = HashMap::new();
        let mut visited_positions = Vec::new();

        while let Some((position, amount_steps)) = queue.pop_front() {
            visited_positions.push(position.clone());

            if amount_steps < 50 {
                add_new_positions(&mut grid, &position, amount_steps, &mut queue, &visited_positions, &mut previous_positions, office_designer_favorite_number);
            }
        }

        visited_positions.len()
    }
}
