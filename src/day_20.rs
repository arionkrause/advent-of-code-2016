pub fn solve(input: &str) {
    // Credits: aurele (https://www.reddit.com/r/adventofcode/comments/5jbeqo/2016_day_20_solutions/dbf1awy)
    println!("Day {}.", file!().chars().filter(|c| c.is_digit(10)).collect::<String>());
    println!("Part 1: {}.", part_1::solve(&input));
    println!("Part 2: {}.", part_2::solve(&input));
    println!();
}

fn get_sorted_blocked_ranges(input: &str) -> Vec<(u64, u64)> {
    let mut blocked_ranges: Vec<_> = input.lines().map(|line| {
        let pair: Vec<_> = line.split('-').map(|value| value.parse::<u64>().unwrap()).collect();
        (pair[0], pair[1])
    }).collect();

    blocked_ranges.sort();
    blocked_ranges
}

mod part_1 {
    use crate::day_20::get_sorted_blocked_ranges;

    pub fn solve(input: &str) -> u64 {
        get_sorted_blocked_ranges(&input).iter().fold(0, |highest, &(low, high)| {
            if low > highest {
                return highest;
            }

            highest.max(high + 1)
        })
    }

    #[cfg(test)]
    #[test]
    fn test_1() {
        let input = "5-8
0-2
4-7";

        assert_eq!(solve(&input), 3);
    }
}

mod part_2 {
    use crate::day_20::get_sorted_blocked_ranges;

    pub fn solve(input: &str) -> u64 {
        get_sorted_blocked_ranges(&input).iter()
                .chain(std::iter::once(&(1 << 32, 1 << 32)))
                .fold((0, 0), |(highest, amount), &(low, high)| {
                    if low > highest {
                        (high + 1, amount + low - highest)
                    } else {
                        (highest.max(high + 1), amount)
                    }
                })
                .1
    }
}
