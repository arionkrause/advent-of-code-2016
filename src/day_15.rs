use regex::Regex;

pub fn solve(input: &str) {
    println!("Day {}.", file!().chars().filter(|c| c.is_digit(10)).collect::<String>());
    println!("Part 1: {}.", part_1::solve(&input));
    println!("Part 2: {}.", part_2::solve(&input));
    println!();
}

#[derive(Debug)]
struct Disc {
    id: usize,
    amount_positions: usize,
    starting_position: usize,
}

fn decode_input(input: &str) -> Vec<Disc> {
    let re = Regex::new(r"^Disc #(?P<id>\d+) has (?P<amount_positions>\d+) positions; at time=0, it is at position (?P<starting_position>\d+)\.$").unwrap();

    input.lines().map(|line| {
        let captures = re.captures(&line).unwrap();

        Disc {
            id: captures.name("id").unwrap().as_str().parse().unwrap(),
            amount_positions: captures.name("amount_positions").unwrap().as_str().parse().unwrap(),
            starting_position: captures.name("starting_position").unwrap().as_str().parse().unwrap(),
        }
    }).collect()
}

fn get_first_valid_time(discs: &Vec<Disc>) -> usize {
    let mut time = 1;

    loop {
        if !discs.iter().any(|disc| (disc.starting_position + disc.id + time) % disc.amount_positions != 0) {
            return time;
        }

        time += 1;
    }
}

mod part_1 {
    use crate::day_15::{decode_input, get_first_valid_time};

    pub fn solve(input: &str) -> usize {
        get_first_valid_time(&decode_input(&input))
    }

    #[cfg(test)]
    #[test]
    fn test_1() {
        let input = "Disc #1 has 5 positions; at time=0, it is at position 4.
Disc #2 has 2 positions; at time=0, it is at position 1.";

        assert_eq!(solve(&input), 5);
    }
}

mod part_2 {
    use crate::day_15::{Disc, decode_input, get_first_valid_time};

    pub fn solve(input: &str) -> usize {
        let mut discs = decode_input(&input);

        let new_disc = Disc {
            id: discs.len() + 1,
            amount_positions: 11,
            starting_position: 0,
        };

        discs.push(new_disc);
        get_first_valid_time(&discs)
    }
}
