#[derive(Debug)]
struct Instruction {
    moves: Vec<Direction>,
}

#[derive(Debug)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

pub fn solve(input: &str) {
    println!("Day {}.", file!().chars().filter(|c| c.is_digit(10)).collect::<String>());
    println!("Part 1: {}.", part_1::solve(&input));
    println!();
}

fn decode_input(input: &str) -> Vec<Instruction> {
    let mut instructions = Vec::new();

    for line in input.lines() {
        let mut instruction = Instruction { moves: vec![] };

        for direction in line.chars() {
            instruction.moves.push(match direction {
                'U' => Direction::Up,
                'R' => Direction::Right,
                'D' => Direction::Down,
                'L' => Direction::Left,
                _ => panic!(),
            });
        }

        instructions.push(instruction);
    }

    instructions
}

mod part_1 {
    use crate::day_2::decode_input;
    use crate::day_2::Direction;

    pub fn solve(input: &str) -> String {
        let instructions = decode_input(&input);
        let mut code = String::new();
        let mut position = 5;

        for instruction in &instructions {
            for a_move in &instruction.moves {
                match a_move {
                    Direction::Up => {
                        match position {
                            1 ... 3 => {}
                            _ => position -= 3,
                        }
                    }
                    Direction::Right => {
                        match position {
                            3 | 6 | 9 => {}
                            _ => position += 1,
                        }
                    }
                    Direction::Down => {
                        match position {
                            7 ... 9 => {}
                            _ => position += 3,
                        }
                    }
                    Direction::Left => {
                        match position {
                            1 | 4 | 7 => {}
                            _ => position -= 1,
                        }
                    }
                }
            }

            code.push(std::char::from_digit(position, 10).unwrap());
        }

        code
    }

    #[cfg(test)]
    #[test]
    fn test_1() {
        let input = "ULL
RRDDD
LURDL
UUUUD";

        assert_eq!(solve(input), "1985");
    }
}
