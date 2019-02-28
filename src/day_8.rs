use regex::Regex;

pub fn solve(input: &str) {
    println!("Day {}.", file!().chars().filter(|c| c.is_digit(10)).collect::<String>());
    println!("Part 1: {}.", part_1::solve(&input));
    println!("Part 2: {}.", part_2::solve(&input));
    println!();
}

#[derive(Debug)]
struct Operation {
    operation_type: OperationType,
    a: u8,
    b: u8,
}

#[derive(Debug)]
enum OperationType {
    Rect,
    RotateColumn,
    RotateRow,
}

fn decode_input(input: &str) -> Vec<Operation> {
    let re_rect = Regex::new(r"^rect (?P<a>\d+)x(?P<b>\d+)$").unwrap();
    let re_rotate_column = Regex::new(r"^rotate column x=(?P<a>\d+) by (?P<b>\d+)$").unwrap();
    let re_rotate_row = Regex::new(r"^rotate row y=(?P<a>\d+) by (?P<b>\d+)$").unwrap();

    input.lines().map(|line| {
        if let Some(captures) = re_rect.captures(&line) {
            Operation {
                operation_type: OperationType::Rect,
                a: captures.name("a").unwrap().as_str().parse().unwrap(),
                b: captures.name("b").unwrap().as_str().parse().unwrap(),
            }
        } else if let Some(captures) = re_rotate_column.captures(&line) {
            Operation {
                operation_type: OperationType::RotateColumn,
                a: captures.name("a").unwrap().as_str().parse().unwrap(),
                b: captures.name("b").unwrap().as_str().parse().unwrap(),
            }
        } else if let Some(captures) = re_rotate_row.captures(&line) {
            Operation {
                operation_type: OperationType::RotateRow,
                a: captures.name("a").unwrap().as_str().parse().unwrap(),
                b: captures.name("b").unwrap().as_str().parse().unwrap(),
            }
        } else {
            panic!();
        }
    }).collect()
}

fn rect(grid: &mut Vec<Vec<char>>, a: u8, b: u8) {
    for row in 0..b {
        for tile in 0..a {
            grid[row as usize][tile as usize] = '#';
        }
    }
}

fn rotate_column(grid: &mut Vec<Vec<char>>, a: u8, b: u8) {
    for _ in 0..b {
        let last_row_tile = grid[5][a as usize];

        for row in (1..=5usize).rev() {
            grid[row][a as usize] = grid[row - 1][a as usize];
        }

        grid[0][a as usize] = last_row_tile;
    }
}

fn rotate_row(grid: &mut Vec<Vec<char>>, a: u8, b: u8) {
    for _ in 0..b {
        let last_tile = grid[a as usize][49];

        for tile in (1..=49usize).rev() {
            grid[a as usize][tile] = grid[a as usize][tile - 1]
        }

        grid[a as usize][0] = last_tile;
    }
}

mod part_1 {
    use crate::day_8::OperationType;
    use crate::day_8::decode_input;
    use crate::day_8::rect;
    use crate::day_8::rotate_column;
    use crate::day_8::rotate_row;

    pub fn solve(input: &str) -> u16 {
        let operations = decode_input(&input);
        let mut grid = vec![vec!['.'; 50]; 6];

        for operation in &operations {
            match operation.operation_type {
                OperationType::Rect => rect(&mut grid, operation.a, operation.b),
                OperationType::RotateColumn => rotate_column(&mut grid, operation.a, operation.b),
                OperationType::RotateRow => rotate_row(&mut grid, operation.a, operation.b),
            }
        }

        grid.iter().map(|row| {
            row.iter().filter(|&tile| tile == &'#').count()
        }).sum::<usize>() as u16
    }

    #[cfg(test)]
    #[test]
    fn test_1() {
        let input = "rect 3x2
rotate column x=1 by 1
rotate row y=0 by 4
rotate column x=1 by 1";

        assert_eq!(solve(&input), 6);
    }
}

mod part_2 {
    use crate::day_8::OperationType;
    use crate::day_8::decode_input;
    use crate::day_8::rect;
    use crate::day_8::rotate_column;
    use crate::day_8::rotate_row;

    pub fn solve(input: &str) -> String {
        let operations = decode_input(&input);
        let mut grid = vec![vec!['.'; 50]; 6];

        for operation in &operations {
            match operation.operation_type {
                OperationType::Rect => rect(&mut grid, operation.a, operation.b),
                OperationType::RotateColumn => rotate_column(&mut grid, operation.a, operation.b),
                OperationType::RotateRow => rotate_row(&mut grid, operation.a, operation.b),
            }
        }

        decode_display(&grid)
    }

    fn decode_display(grid: &Vec<Vec<char>>) -> String {
        let mut result = String::new();

        for tile in (0..46usize).step_by(5) {
            let mut letter_encoded = String::new();

            for row in 0..6usize {
                for offset in 0..5 {
                    letter_encoded.push(grid[row][tile + offset]);
                }
            }

            // Commented letters are (probably) never used because they are too wide or ambiguous
            result.push(match letter_encoded.as_ref() {
                ".##..#..#.#..#.####.#..#.#..#." => 'A',
                "###..#..#.###..#..#.#..#.###.." => 'B',
                ".##..#..#.#....#....#..#..##.." => 'C',
                "###..#..#.#..#.#..#.#..#.###.." => 'D',
                "####.#....###..#....#....####." => 'E',
                "####.#....###..#....#....#...." => 'F',
                ".##..#..#.#....#.##.#..#..###." => 'G',
                "#..#.#..#.####.#..#.#..#.#..#." => 'H',
                ".###...#....#....#....#...###." => 'I',
                "..##....#....#....#.#..#..##.." => 'J',
                "#..#.#.#..##...#.#..#.#..#..#." => 'K',
                "#....#....#....#....#....####." => 'L',
//                "______________________________" => 'M',
//                "______________________________" => 'N',
                ".##..#..#.#..#.#..#.#..#..##.." => 'O',
                "###..#..#.#..#.###..#....#...." => 'P',
//                "______________________________" => 'Q',
                "###..#..#.#..#.###..#.#..#..#." => 'R',
                ".###.#....#.....##.....#.###.." => 'S',
//                "______________________________" => 'T',
                "#..#.#..#.#..#.#..#.#..#..##.." => 'U',
//                "______________________________" => 'V',
//                "______________________________" => 'W',
//                "______________________________" => 'X',
                "#...##...#.#.#...#....#....#.." => 'Y',
                "####....#...#...#...#....####." => 'Z',
                _ => panic!(),
            })
        }

        result
    }
}
