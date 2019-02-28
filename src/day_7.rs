use regex::Regex;

pub fn solve(input: &str) {
    println!("Day {}.", file!().chars().filter(|c| c.is_digit(10)).collect::<String>());
    println!("Part 1: {}.", part_1::solve(&input));
    println!();
}

#[derive(Debug)]
struct IpAddress {
    canonical_address: String,
    abba_sequences: Vec<String>,
    hypernet_sequences: Vec<String>,
}

impl IpAddress {
    fn supports_tls(&self) -> bool {
        !self.abba_sequences.is_empty()
                && self.hypernet_sequences.iter().all(|hypernet_sequence| !contains_abba_sequence(&hypernet_sequence))
    }
}

fn decode_input(input: &str) -> Vec<IpAddress> {
    let re = Regex::new(r"(?P<sequence>[a-z]+)(\[(?P<hypernet_sequence>[a-z]+)])?").unwrap();

    input.lines().map(|line| {
        let mut ip_address = IpAddress {
            canonical_address: line.to_owned(),
            abba_sequences: vec![],
            hypernet_sequences: vec![],
        };

        re.captures_iter(&line).for_each(|captures| {
            let sequence = captures.name("sequence").unwrap().as_str();
            ip_address.abba_sequences.extend(get_abba_sequences(&sequence));

            if let Some(hypernet_sequence) = captures.name("hypernet_sequence") {
                ip_address.hypernet_sequences.push(hypernet_sequence.as_str().to_string());
            }
        });

        ip_address
    }).collect()
}

fn contains_abba_sequence(sequence: &str) -> bool {
    sequence.chars().collect::<Vec<char>>().windows(4).any(|window| {
        window[0] == window[3]
                && window[1] == window[2]
                && window[0] != window[1]
    })
}

fn get_abba_sequences(sequence: &str) -> Vec<String> {
    sequence.chars().collect::<Vec<char>>().windows(4).filter(|window| {
        window[0] == window[3]
                && window[1] == window[2]
                && window[0] != window[1]
    })
            .map(|window| window.into_iter().collect())
            .collect()
}

mod part_1 {
    use crate::day_7::decode_input;

    pub fn solve(input: &str) -> u16 {
        decode_input(&input).iter().filter(|ip_address| {
            ip_address.supports_tls()
        }).count() as u16
    }

    #[cfg(test)]
    #[test]
    fn test_1() {
        assert_eq!(solve("abba[mnop]qrst"), 1);
    }

    #[test]
    fn test_2() {
        assert_eq!(solve("abcd[bddb]xyyx"), 0);
    }

    #[test]
    fn test_3() {
        assert_eq!(solve("aaaa[qwer]tyui"), 0);
    }

    #[test]
    fn test_4() {
        assert_eq!(solve("ioxxoj[asdfgh]zxcvbn"), 1);
    }
}
