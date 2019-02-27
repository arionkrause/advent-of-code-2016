#[derive(Debug)]
struct Triangle {
    side_1: u16,
    side_2: u16,
    side_3: u16,
}

pub fn solve(input: &str) {
    println!("Day {}.", file!().chars().filter(|c| c.is_digit(10)).collect::<String>());
    println!("Part 1: {}.", part_1::solve(&input));
    println!("Part 2: {}.", part_2::solve(&input));
    println!();
}

fn get_amount_possible_triangles(triangles: &Vec<Triangle>) -> u16 {
    triangles.iter()
            .filter(|triangle| {
                triangle.side_1 + triangle.side_2 > triangle.side_3
                        && triangle.side_1 + triangle.side_3 > triangle.side_2
                        && triangle.side_2 + triangle.side_3 > triangle.side_1
            })
            .count() as u16
}

mod part_1 {
    use regex::Regex;
    use crate::day_3::Triangle;
    use crate::day_3::get_amount_possible_triangles;

    pub fn solve(input: &str) -> u16 {
        get_amount_possible_triangles(&decode_input(&input))
    }

    fn decode_input(input: &str) -> Vec<Triangle> {
        let re = Regex::new(r"^ *(?P<side_1>\d+) *(?P<side_2>\d+) *(?P<side_3>\d+)$").unwrap();

        input.lines().map(|line| {
            let captures = re.captures(&line).unwrap();

            Triangle {
                side_1: captures.name("side_1").unwrap().as_str().parse().unwrap(),
                side_2: captures.name("side_2").unwrap().as_str().parse().unwrap(),
                side_3: captures.name("side_3").unwrap().as_str().parse().unwrap(),
            }
        }).collect()
    }

    #[cfg(test)]
    #[test]
    fn test_1() {
        assert_eq!(solve("5 10 25"), 0);
    }
}

mod part_2 {
    use regex::Regex;
    use crate::day_3::Triangle;
    use crate::day_3::get_amount_possible_triangles;

    pub fn solve(input: &str) -> u16 {
        get_amount_possible_triangles(&decode_input(&input))
    }

    fn decode_input(input: &str) -> Vec<Triangle> {
        let mut triangles = Vec::new();
        let re = Regex::new(r"^ *(?P<side_1>\d+) *(?P<side_2>\d+) *(?P<side_3>\d+)$").unwrap();
        let mut buffer = Vec::with_capacity(9);

        for (index, line) in input.lines().enumerate() {
            let captures = re.captures(&line).unwrap();
            buffer.push(captures.name("side_1").unwrap().as_str().parse().unwrap());
            buffer.push(captures.name("side_2").unwrap().as_str().parse().unwrap());
            buffer.push(captures.name("side_3").unwrap().as_str().parse().unwrap());

            if (index + 1) % 3 == 0 {
                triangles.push(Triangle {
                    side_1: buffer[0],
                    side_2: buffer[3],
                    side_3: buffer[6],
                });

                triangles.push(Triangle {
                    side_1: buffer[1],
                    side_2: buffer[4],
                    side_3: buffer[7],
                });

                triangles.push(Triangle {
                    side_1: buffer[2],
                    side_2: buffer[5],
                    side_3: buffer[8],
                });

                buffer.clear();
            }
        }

        triangles
    }

    #[cfg(test)]
    #[test]
    fn test_1() {
        let input = "101 301 501
102 302 502
103 303 503
201 401 601
202 402 602
203 403 603";

        assert_eq!(solve(&input), 6);
    }
}
