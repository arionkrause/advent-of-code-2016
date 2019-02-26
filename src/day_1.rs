use regex::Regex;

pub fn solve(input: &str) {
    println!("Day {}.", file!().chars().filter(|c| c.is_digit(10)).collect::<String>());
    println!("Part 1: {}.", part_1::solve(&input));
    println!();
}

#[derive(Debug)]
struct Instruction {
    direction: Direction,
    blocks: u8,
}

#[derive(Debug)]
struct State {
    facing: CardinalDirection,
    y: i16,
    x: i16,
}

#[derive(Debug)]
enum CardinalDirection {
    North,
    East,
    South,
    West,
}

#[derive(Debug)]
enum Direction {
    Left,
    Right,
}

fn decode_input(input: &str) -> Vec<Instruction> {
    let mut instructions = Vec::new();
    let re = Regex::new(r"((?P<direction>[LR])(?P<blocks>\d+)(, )?)").unwrap();

    for captures in re.captures_iter(&input) {
        let direction = match captures.name("direction").unwrap().as_str() {
            "L" => Direction::Left,
            "R" => Direction::Right,
            _ => panic!(),
        };

        let blocks: u8 = captures.name("blocks").unwrap().as_str().parse().unwrap();
        instructions.push(Instruction { direction, blocks });
    }

    instructions
}

mod part_1 {
    use crate::day_1::decode_input;
    use crate::day_1::CardinalDirection;
    use crate::day_1::Direction;
    use crate::day_1::State;

    pub fn solve(input: &str) -> u16 {
        let instructions = decode_input(&input);

        let mut state = State {
            facing: CardinalDirection::North,
            y: 0,
            x: 0
        };

        for instruction in &instructions {
            match instruction.direction {
                Direction::Left => {
                    match state.facing {
                        CardinalDirection::North => {
                            state.facing = CardinalDirection::West;
                            state.x -= instruction.blocks as i16;
                        },
                        CardinalDirection::East => {
                            state.facing = CardinalDirection::North;
                            state.y += instruction.blocks as i16;
                        },
                        CardinalDirection::South => {
                            state.facing = CardinalDirection::East;
                            state.x += instruction.blocks as i16;
                        },
                        CardinalDirection::West => {
                            state.facing = CardinalDirection::South;
                            state.y -= instruction.blocks as i16;
                        },
                    }
                },
                Direction::Right => {
                    match state.facing {
                        CardinalDirection::North => {
                            state.facing = CardinalDirection::East;
                            state.x += instruction.blocks as i16;
                        },
                        CardinalDirection::East => {
                            state.facing = CardinalDirection::South;
                            state.y -= instruction.blocks as i16;
                        },
                        CardinalDirection::South => {
                            state.facing = CardinalDirection::West;
                            state.x -= instruction.blocks as i16;
                        },
                        CardinalDirection::West => {
                            state.facing = CardinalDirection::North;
                            state.y += instruction.blocks as i16;
                        },
                    }
                },
            }
        }

        state.y.abs() as u16 + state.x.abs() as u16
    }

    #[cfg(test)]
    #[test]
    fn test_1() {
        assert_eq!(solve("R2, L3"), 5);
    }

    #[test]
    fn test_2() {
        assert_eq!(solve("R2, R2, R2"), 2);
    }

    #[test]
    fn test_3() {
        assert_eq!(solve("R5, L5, R5, R3"), 12);
    }
}
