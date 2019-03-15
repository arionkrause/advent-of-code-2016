pub fn solve(input: &str) {
    println!("Day {}.", file!().chars().filter(|c| c.is_digit(10)).collect::<String>());
    println!("Part 1: {}.", part_1::solve(&input));
    println!();
}

mod part_1 {
    use regex::Regex;

    #[derive(Debug)]
    struct Node {
        available: usize,
        used: usize,
    }

    fn decode_input(input: &str) -> Vec<Node> {
        let re = Regex::new(r"^/dev/grid/node-x(?P<x>\d+)-y(?P<y>\d+) +\d+T +(?P<used>\d+)T +(?P<available>\d+)T +\d+%$").unwrap();

        input.lines().skip(2).map(|line| {
            let captures = re.captures(&line).unwrap();

            Node {
                available: captures.name("available").unwrap().as_str().parse().unwrap(),
                used: captures.name("used").unwrap().as_str().parse().unwrap(),
            }
        }).collect()
    }

    pub fn solve(input: &str) -> usize {
        let nodes = decode_input(&input);
        let mut viable_pairs = 0;

        for (index, node) in nodes.iter().enumerate() {
            for node_other in nodes[index + 1..].iter() {
                if node.used > 0 && node.used <= node_other.available
                        || node_other.used > 0 && node_other.used <= node.available{
                    viable_pairs += 1;
                }
            }
        }

        viable_pairs
    }
}
