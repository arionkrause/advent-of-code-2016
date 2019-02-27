use regex::Regex;

#[derive(Debug)]
struct Triangle {
    side_1: u16,
    side_2: u16,
    side_3: u16,
}

pub fn solve(input: &str) {
    println!("Day {}.", file!().chars().filter(|c| c.is_digit(10)).collect::<String>());
    println!("Part 1: {}.", part_1::solve(&input));
    println!();
}

fn decode_input(input: &str) -> Vec<Triangle> {
    let mut triangles = Vec::new();
    let re = Regex::new(r"^ *(?P<side_1>\d+) *(?P<side_2>\d+) *(?P<side_3>\d+)$").unwrap();

    for line in input.lines() {
        let captures = re.captures(&line).unwrap();

        triangles.push(Triangle {
            side_1: captures.name("side_1").unwrap().as_str().parse().unwrap(),
            side_2: captures.name("side_2").unwrap().as_str().parse().unwrap(),
            side_3: captures.name("side_3").unwrap().as_str().parse().unwrap(),
        })
    }

    triangles
}

mod part_1 {
    use crate::day_3::decode_input;

    pub fn solve(input: &str) -> u16 {
        decode_input(&input).iter()
                .filter(|triangle| {
                    triangle.side_1 + triangle.side_2 > triangle.side_3
                            && triangle.side_1 + triangle.side_3 > triangle.side_2
                            && triangle.side_2 + triangle.side_3 > triangle.side_1
                })
                .count() as u16
    }

    #[cfg(test)]
    #[test]
    fn test_1() {
        assert_eq!(solve("5 10 25"), 0);
    }
}
