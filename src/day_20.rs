pub fn solve(input: &str) {
    println!("Day {}.", file!().chars().filter(|c| c.is_digit(10)).collect::<String>());
    println!("Part 1: {}.", part_1::solve(&input));
    println!();
}

#[derive(Debug, Eq, Ord, PartialEq, PartialOrd)]
struct Range {
    start: u32,
    end: u32,
}

mod part_1 {
    use crate::day_20::Range;

    pub fn solve(input: &str) -> u32 {
        let mut blocked_ranges: Vec<Range> = Vec::new();

        'line: for line in input.lines() {
            let range: Vec<&str> = line.split('-').collect();

            let new_range = Range {
                start: range[0].parse().unwrap(),
                end: range[1].parse().unwrap(),
            };

            blocked_ranges.push(new_range);
            join_ranges(&mut blocked_ranges);
        }

        blocked_ranges.sort();

        if blocked_ranges[0].start == 0 {
            return blocked_ranges[0].end + 1;
        }

        0
    }

    fn join_ranges(ranges: &mut Vec<Range>) {
        loop {
            let mut join = None;
            let mut remove = None;

            'range: for (index, range) in ranges.iter().enumerate() {
                for (index_other, other_range) in ranges.iter().enumerate() {
                    if index == index_other {
                        continue;
                    }

                    if range.start > 0
                            && other_range.start < range.start
                            && other_range.end >= range.start - 1
                            && other_range.end < range.end {
                        join = Some((index, index_other));
                        break 'range;
                    }

                    if range.start >= other_range.start
                            && range.end <= other_range.end {
                        remove = Some(index);
                    }
                }
            }

            match join {
                Some((index, index_other)) => ranges[index].start = ranges[index_other].start,
                None => {
                    match remove {
                        Some(index) => {
                            ranges.remove(index);
                        }
                        None => return,
                    }
                }
            }
        }
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
