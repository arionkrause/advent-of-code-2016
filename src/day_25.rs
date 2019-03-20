use regex::Regex;

pub fn solve(input: &str) {
    println!("Day {}.", file!().chars().filter(|c| c.is_digit(10)).collect::<String>());
    println!("Part 1: {}.", part_1::solve(&input));
    println!();
}

#[derive(Debug)]
struct Instruction {
    opcode: Opcode,
    operand_a: String,
    operand_b: Option<String>,
}

#[derive(Debug)]
enum Opcode {
    Cpy,
    Dec,
    Inc,
    Jnz,
    Out,
}

fn decode_input(input: &str) -> Vec<Instruction> {
    let mut instructions = Vec::with_capacity(input.lines().count());
    let re_cpy = Regex::new(r"^cpy (?P<operand_a>-?[a-d0-9]+) (?P<operand_b>[a-d])$").unwrap();
    let re_dec_or_inc = Regex::new(r"^(?P<opcode>dec|inc) (?P<operand_a>[a-d])$").unwrap();
    let re_jnz = Regex::new(r"^jnz (?P<operand_a>-?[a-d0-9]+) (?P<operand_b>-?[a-d0-9]+)$").unwrap();
    let re_out = Regex::new(r"^out (?P<operand_a>[a-d])$").unwrap();

    for line in input.lines() {
        if let Some(captures) = re_cpy.captures(&line) {
            instructions.push(Instruction {
                opcode: Opcode::Cpy,
                operand_a: captures.name("operand_a").unwrap().as_str().to_string(),
                operand_b: Some(captures.name("operand_b").unwrap().as_str().to_string()),
            });

            continue;
        }

        if let Some(captures) = re_dec_or_inc.captures(&line) {
            instructions.push(Instruction {
                opcode: match captures.name("opcode").unwrap().as_str() {
                    "dec" => Opcode::Dec,
                    "inc" => Opcode::Inc,
                    _ => panic!(),
                },
                operand_a: captures.name("operand_a").unwrap().as_str().to_string(),
                operand_b: None,
            });

            continue;
        }

        if let Some(captures) = re_jnz.captures(&line) {
            instructions.push(Instruction {
                opcode: Opcode::Jnz,
                operand_a: captures.name("operand_a").unwrap().as_str().to_string(),
                operand_b: Some(captures.name("operand_b").unwrap().as_str().to_string()),
            });

            continue;
        }

        if let Some(captures) = re_out.captures(&line) {
            instructions.push(Instruction {
                opcode: Opcode::Out,
                operand_a: captures.name("operand_a").unwrap().as_str().to_string(),
                operand_b: None,
            });

            continue;
        }

        panic!();
    }

    instructions
}

fn run(registers: &mut [isize], instructions: &mut Vec<Instruction>) -> Vec<isize> {
    let mut signal = Vec::new();
    let mut program_counter: isize = 0;

    loop {
        let instruction_index = program_counter as usize;
        program_counter += 1;

        match instructions[instruction_index].opcode {
            Opcode::Cpy => {
                let value = match instructions[instruction_index].operand_a.parse::<isize>() {
                    Ok(value) => value,
                    Err(_) => registers[instructions[instruction_index].operand_a.chars().next().unwrap() as usize - 97],
                };

                registers[instructions[instruction_index].operand_b.clone().unwrap().chars().next().unwrap() as usize - 97] = value;
            }
            Opcode::Dec => registers[instructions[instruction_index].operand_a.chars().next().unwrap() as usize - 97] -= 1,
            Opcode::Inc => registers[instructions[instruction_index].operand_a.chars().next().unwrap() as usize - 97] += 1,
            Opcode::Jnz => {
                let offset = match instructions[instruction_index].operand_b.clone().unwrap().parse::<isize>() {
                    Ok(value) => value,
                    Err(_) => registers[instructions[instruction_index].operand_b.clone().unwrap().chars().next().unwrap() as usize - 97],
                };

                match instructions[instruction_index].operand_a.parse::<isize>() {
                    Ok(value) => {
                        if value != 0 {
                            program_counter += offset - 1;
                        }
                    }
                    Err(_) => {
                        if registers[instructions[instruction_index].operand_a.chars().next().unwrap() as usize - 97] != 0 {
                            program_counter += offset - 1;
                        }
                    }
                }
            }
            Opcode::Out => {
                signal.push(registers[instructions[instruction_index].operand_a.chars().next().unwrap() as usize - 97]);

                if signal.len() == 8 {
                    return signal;
                }
            }
        }

        if program_counter < 0 || program_counter as usize >= instructions.len() {
            break;
        }
    }

    panic!();
}

mod part_1 {
    use crate::day_25::{decode_input, run};

    pub fn solve(input: &str) -> usize {
        let mut registers = [0; 4];

        for index in 0..std::usize::MAX {
            registers[0] = index as isize;
            let mut instructions = decode_input(&input);
            let signal = run(&mut registers, &mut instructions);

            if signal.iter().enumerate().all(|(index, value)|
                (index % 2 == 0 && *value == 0)
                    || (index % 2 == 1 && *value == 1)) {
                return index;
            }
        }

        panic!();
    }
}
