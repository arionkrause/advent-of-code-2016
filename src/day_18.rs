pub fn solve(input: &str) {
    println!("Day {}.", file!().chars().filter(|c| c.is_digit(10)).collect::<String>());
    println!("Part 1: {}.", part_1::solve(&input, 40));
    println!();
}

mod part_1 {
    pub fn solve(input: &str, amount_rows: usize) -> usize {
        let mut grid: Vec<Vec<char>> = Vec::with_capacity(amount_rows);
        grid.push(input.chars().collect());
        let amount_tiles_per_row = grid[0].len();

        for _ in 0..amount_rows - 1 {
            let previous_row_index = grid.len() - 1;
            let mut row = Vec::with_capacity(amount_tiles_per_row);

            for index in 0..amount_tiles_per_row {
                let previous_row_left = if index == 0 || grid[previous_row_index][index - 1] == '.' { '.' } else { '^' };
                let previous_row_center = if grid[previous_row_index][index] == '.' { '.' } else { '^' };
                let previous_row_right = if index == amount_tiles_per_row - 1 || grid[previous_row_index][index + 1] == '.' { '.' } else { '^' };

                row.push(if (previous_row_left == '^' && previous_row_center == '^' && previous_row_right == '.')
                        || (previous_row_left == '.' && previous_row_center == '^' && previous_row_right == '^')
                        || (previous_row_left == '^' && previous_row_center == '.' && previous_row_right == '.')
                        || (previous_row_left == '.' && previous_row_center == '.' && previous_row_right == '^') {
                    '^'
                } else {
                    '.'
                });
            }

            grid.push(row);
        }

        grid.iter().flatten().filter(|&&tile| tile == '.').count()
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
