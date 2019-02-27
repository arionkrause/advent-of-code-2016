pub fn solve(input: &str) {
    println!("Day {}.", file!().chars().filter(|c| c.is_digit(10)).collect::<String>());
    println!("Part 1: {}.", part_1::solve(&input));
    println!();
}

#[derive(Clone, Debug)]
struct LetterFrequency {
    letter: char,
    frequency: u8,
}

mod part_1 {
    use std::collections::HashMap;
    use crate::day_6::LetterFrequency;

    pub fn solve(input: &str) -> String {
        let mut letters_frequencies: Vec<HashMap<char, LetterFrequency>> = vec![HashMap::new(); input.lines().next().unwrap().len()];

        for line in input.lines() {
            for (index, letter) in line.chars().enumerate() {
                letters_frequencies[index].entry(letter)
                        .and_modify(|letter_frequency| letter_frequency.frequency += 1)
                        .or_insert(LetterFrequency { letter, frequency: 1 });
            }
        }

        letters_frequencies.into_iter()
                .map(|letter_frequency| {
                    let mut vector_letter_frequency = letter_frequency.into_iter()
                            .map(|(_, letter_frequency)| letter_frequency)
                            .collect::<Vec<LetterFrequency>>();

                    vector_letter_frequency.sort_by(|a, b| b.frequency.cmp(&a.frequency));
                    vector_letter_frequency[0].letter
                }).collect()
    }

    #[cfg(test)]
    #[test]
    fn test_1() {
        let input = "eedadn
drvtee
eandsr
raavrd
atevrs
tsrnev
sdttsa
rasrtv
nssdts
ntnada
svetve
tesnvt
vntsnd
vrdear
dvrsen
enarar";

        assert_eq!(solve(&input), "easter");
    }
}
