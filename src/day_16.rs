pub fn solve(input: &str) {
    println!("Day {}.", file!().chars().filter(|c| c.is_digit(10)).collect::<String>());
    println!("Part 1: {}.", part_1::solve(&input, 272));
    println!("Part 2: {}.", part_2::solve(&input, 35651584));
    println!();
}

fn get_dragon_curve(data: &str, length: usize) -> Vec<char> {
    let mut data_a: Vec<char> = data.chars().collect();

    while data_a.len() < length {
        let mut data_b: Vec<char> = data_a.clone().into_iter().rev().collect();

        for i in 0..data_b.len() {
            data_b[i] = if data_b[i] == '1' { '0' } else { '1' };
        }

        data_a.push('0');
        data_a.extend(data_b);
    }

    data_a.truncate(length);
    data_a
}

fn get_checksum(data: &Vec<char>) -> String {
    let mut checksum: Vec<char> = data.clone();
    let mut buffer = Vec::new();

    loop {
        for index in (0..checksum.len() - 1).step_by(2) {
            buffer.push(if checksum[index] == checksum[index + 1] { '1' } else { '0' });
        }

        if buffer.len() % 2 == 1 {
            return buffer.into_iter().collect();
        }

        std::mem::swap(&mut checksum, &mut buffer);
        buffer.clear();
    }
}

mod part_1 {
    use crate::day_16::{get_checksum, get_dragon_curve};

    pub fn solve(input: &str, disk_length: usize) -> String {
        get_checksum(&get_dragon_curve(&input, disk_length))
    }

    #[cfg(test)]
    #[test]
    fn test_1() {
        assert_eq!(solve("110010110100", 12), "100");
    }

    #[test]
    fn test_2() {
        assert_eq!(solve("10000", 20), "01100");
    }
}

mod part_2 {
    use crate::day_16::{get_checksum, get_dragon_curve};

    pub fn solve(input: &str, disk_length: usize) -> String {
        get_checksum(&get_dragon_curve(&input, disk_length))
    }
}
