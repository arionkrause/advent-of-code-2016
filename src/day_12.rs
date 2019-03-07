use regex::Regex;

pub fn solve(input: &str) {
    println!("Day {}.", file!().chars().filter(|c| c.is_digit(10)).collect::<String>());
    println!("Part 1: {}.", part_1::solve(&input));
    println!();
}

#[derive(Debug)]
struct Instruction {
    opcode: Opcode,
    operand_a: usize,
    operand_b: Option<isize>,
}

#[derive(Debug)]
enum Opcode {
    Cpyr,
    Cpyv,
    Decv,
    Incv,
    Jnzr,
    Jnzv,
}

fn decode_input(input: &str) -> Vec<Instruction> {
    let mut instructions = Vec::with_capacity(input.lines().count());
    let re_cpyr = Regex::new(r"^cpy (?P<operand_a>[a-d]) (?P<operand_b>[a-d])$").unwrap();
    let re_cpyv = Regex::new(r"^cpy (?P<operand_a>\d+) (?P<operand_b>[a-d])$").unwrap();
    let re_decv_or_incv = Regex::new(r"^(?P<opcode>dec|inc) (?P<operand_a>[a-d])$").unwrap();
    let re_jnzr = Regex::new(r"^jnz (?P<operand_a>[a-d]) (?P<operand_b>-?\d+)$").unwrap();
    let re_jnzv = Regex::new(r"^jnz (?P<operand_a>\d+) (?P<operand_b>-?\d+)$").unwrap();

    for line in input.lines() {
        if let Some(captures) = re_cpyr.captures(&line) {
            instructions.push(Instruction {
                opcode: Opcode::Cpyr,
                operand_a: (captures.name("operand_a").unwrap().as_str().chars().next().unwrap() as u8 - 97) as usize,
                operand_b: Some((captures.name("operand_b").unwrap().as_str().chars().next().unwrap() as u8 - 97) as isize),
            });

            continue;
        }

        if let Some(captures) = re_cpyv.captures(&line) {
            instructions.push(Instruction {
                opcode: Opcode::Cpyv,
                operand_a: captures.name("operand_a").unwrap().as_str().parse().unwrap(),
                operand_b: Some((captures.name("operand_b").unwrap().as_str().chars().next().unwrap() as u8 - 97) as isize),
            });

            continue;
        }

        if let Some(captures) = re_decv_or_incv.captures(&line) {
            instructions.push(Instruction {
                opcode: match captures.name("opcode").unwrap().as_str() {
                    "dec" => Opcode::Decv,
                    "inc" => Opcode::Incv,
                    _ => panic!(),
                },
                operand_a: (captures.name("operand_a").unwrap().as_str().chars().next().unwrap() as u8 - 97) as usize,
                operand_b: None,
            });

            continue;
        }

        if let Some(captures) = re_jnzr.captures(&line) {
            instructions.push(Instruction {
                opcode: Opcode::Jnzr,
                operand_a: (captures.name("operand_a").unwrap().as_str().chars().next().unwrap() as u8 - 97) as usize,
                operand_b: Some(captures.name("operand_b").unwrap().as_str().parse().unwrap())
            });

            continue;
        }

        if let Some(captures) = re_jnzv.captures(&line) {
            instructions.push(Instruction {
                opcode: Opcode::Jnzv,
                operand_a: captures.name("operand_a").unwrap().as_str().parse().unwrap(),
                operand_b: Some(captures.name("operand_b").unwrap().as_str().parse().unwrap())
            });

            continue;
        }

        panic!();
    }

    instructions
}

mod part_1 {
    use crate::day_12::{decode_input, Opcode};

    pub fn solve(input: &str) -> usize {
        let mut registers = [0; 4];
        let instructions = decode_input(&input);
        let mut program_counter: isize = 0;

        loop {
            let instruction = &instructions[program_counter as usize];
            program_counter += 1;

            match instruction.opcode {
                Opcode::Cpyr => registers[instruction.operand_b.unwrap() as usize] = registers[instruction.operand_a as usize],
                Opcode::Cpyv => registers[instruction.operand_b.unwrap() as usize] = instruction.operand_a as usize,
                Opcode::Decv => registers[instruction.operand_a as usize] -= 1,
                Opcode::Incv => registers[instruction.operand_a as usize] += 1,
                Opcode::Jnzr => if registers[instruction.operand_a as usize] != 0 {
                    program_counter += instruction.operand_b.unwrap() - 1;
                },
                Opcode::Jnzv => if instruction.operand_a != 0 {
                    program_counter += instruction.operand_b.unwrap() - 1;
                },
            }

            if program_counter < 0 || program_counter as usize >= instructions.len() {
                break;
            }
        }

        registers[0]
    }

    #[cfg(test)]
    #[test]
    fn test_1() {
        let input = "cpy 41 a
inc a
inc a
dec a
jnz a 2
dec a";

        assert_eq!(solve(&input), 42);
    }
}
