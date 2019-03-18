use regex::Regex;

pub fn solve(input: &str) {
    println!("Day {}.", file!().chars().filter(|c| c.is_digit(10)).collect::<String>());
    println!("Part 1: {}.", part_1::solve(&input, 7));
    println!("Part 2: {}.", part_2::solve(&input, 12));
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
    Tgl,
}

fn decode_input(input: &str) -> Vec<Instruction> {
    let mut instructions = Vec::with_capacity(input.lines().count());
    let re_cpy = Regex::new(r"^cpy (?P<operand_a>-?[a-d0-9]+) (?P<operand_b>[a-d])$").unwrap();
    let re_dec_or_inc = Regex::new(r"^(?P<opcode>dec|inc) (?P<operand_a>[a-d])$").unwrap();
    let re_jnz = Regex::new(r"^jnz (?P<operand_a>-?[a-d0-9]+) (?P<operand_b>-?[a-d0-9]+)$").unwrap();
    let re_tgl = Regex::new(r"^tgl (?P<operand_a>[a-d])$").unwrap();

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

        if let Some(captures) = re_tgl.captures(&line) {
            instructions.push(Instruction {
                opcode: Opcode::Tgl,
                operand_a: captures.name("operand_a").unwrap().as_str().to_string(),
                operand_b: None,
            });

            continue;
        }

        panic!();
    }

    instructions
}

fn run(registers: &mut [isize], instructions: &mut Vec<Instruction>) -> isize {
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
            Opcode::Tgl => {
                let offset = registers[instructions[instruction_index].operand_a.chars().next().unwrap() as usize - 97];
                let toggled_instruction_index = instruction_index as isize + offset;

                if toggled_instruction_index < 0 || toggled_instruction_index as usize >= instructions.len() {
                    continue;
                }

                let toggled_instruction_index = toggled_instruction_index as usize;
                let instruction = &mut instructions[toggled_instruction_index];

                match instruction.opcode {
                    Opcode::Cpy => instruction.opcode = Opcode::Jnz,
                    Opcode::Dec => instruction.opcode = Opcode::Inc,
                    Opcode::Inc => instruction.opcode = Opcode::Dec,
                    Opcode::Jnz => instruction.opcode = Opcode::Cpy,
                    Opcode::Tgl => instruction.opcode = Opcode::Inc,
                }
            }
        }

        if program_counter < 0 || program_counter as usize >= instructions.len() {
            break;
        }
    }

    registers[0]
}

fn bypass_run(input: &str, initial_value_register_a: isize) -> isize {
    let value_c: isize = input.lines().nth(19).unwrap().split_whitespace().nth(1).unwrap().parse().unwrap();
    let value_d: isize = input.lines().nth(20).unwrap().split_whitespace().nth(1).unwrap().parse().unwrap();
    return (2..=initial_value_register_a).fold(1, |product, number| product * number) + value_c * value_d
}

mod part_1 {
    use crate::day_23::{bypass_run, decode_input, run};

    pub fn solve(input: &str, initial_value_register_a: isize) -> isize {
        if input.lines().count() == 26 {
            return bypass_run(&input, initial_value_register_a);
        }

        let mut registers = [0; 4];
        registers[0] = initial_value_register_a;
        let mut instructions = decode_input(&input);
        run(&mut registers, &mut instructions)
    }

    #[cfg(test)]
    #[test]
    fn test_1() {
        let input = "cpy 2 a
tgl a
tgl a
tgl a
cpy 1 a
dec a
dec a";

        assert_eq!(solve(&input, 0), 3);
    }
}

mod part_2 {
    use crate::day_23::bypass_run;

    pub fn solve(input: &str, initial_value_register_a: isize) -> isize {
        return bypass_run(&input, initial_value_register_a);
    }
}
