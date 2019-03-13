pub fn solve(input: &str) {
    println!("Day {}.", file!().chars().filter(|c| c.is_digit(10)).collect::<String>());
    println!("Part 1: {}.", part_1::solve(&input, 40));
    println!("Part 2: {}.", part_2::solve(&input, 400_000));
    println!();
}

fn get_amount_safe_tiles(input: &str, amount_rows: usize) -> usize {
    let amount_tiles_per_row = input.len();
    let mut tiles: Vec<char> = input.chars().collect();
    let mut buffer: Vec<char> = vec!['.'; amount_tiles_per_row];
    let mut amount_safe_tiles = tiles.iter().filter(|&&tile| tile == '.').count();

    for _ in 0..amount_rows - 1 {
        for index in 0..amount_tiles_per_row {
            let previous_row_left = if index == 0 || tiles[index - 1] == '.' { '.' } else { '^' };
            let previous_row_center = if tiles[index] == '.' { '.' } else { '^' };
            let previous_row_right = if index == amount_tiles_per_row - 1 || tiles[index + 1] == '.' { '.' } else { '^' };

            buffer[index] = if (previous_row_left == '^' && previous_row_center == '^' && previous_row_right == '.')
                    || (previous_row_left == '.' && previous_row_center == '^' && previous_row_right == '^')
                    || (previous_row_left == '^' && previous_row_center == '.' && previous_row_right == '.')
                    || (previous_row_left == '.' && previous_row_center == '.' && previous_row_right == '^') {
                '^'
            } else {
                '.'
            };
        }

        amount_safe_tiles += buffer.iter().filter(|&&tile| tile == '.').count();
        std::mem::swap(&mut tiles, &mut buffer);
    }

    amount_safe_tiles
}

mod part_1 {
    use crate::day_18::get_amount_safe_tiles;

    pub fn solve(input: &str, amount_rows: usize) -> usize {
        get_amount_safe_tiles(&input, amount_rows)
    }

    #[cfg(test)]
    #[test]
    fn test_1() {
        assert_eq!(solve("..^^.", 3), 6);
    }

    #[test]
    fn test_2() {
        assert_eq!(solve(".^^.^.^^^^", 10), 38);
    }
}

mod part_2 {
    use crate::day_18::get_amount_safe_tiles;

    pub fn solve(input: &str, amount_rows: usize) -> usize {
        get_amount_safe_tiles(&input, amount_rows)
    }
}
