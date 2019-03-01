use regex::Regex;
use std::collections::HashMap;

pub fn solve(input: &str) {
    println!("Day {}.", file!().chars().filter(|c| c.is_digit(10)).collect::<String>());
    println!("Part 1: {}.", part_1::solve(&input, 17, 61));
    println!("Part 2: {}.", part_2::solve(&input));
    println!();
}

#[derive(Debug)]
struct Bot {
    value_low: Option<u8>,
    value_high: Option<u8>,
    bot_id_gives_low_to: Option<usize>,
    output_id_gives_low_to: Option<usize>,
    bot_id_gives_high_to: Option<usize>,
    output_id_gives_high_to: Option<usize>,
}

impl Bot {
    fn attribute_value(&mut self, value: u8) {
        match self.value_low {
            Some(value_low) => {
                if value < value_low {
                    self.value_high = self.value_low;
                    self.value_low = Some(value);
                } else {
                    self.value_high = Some(value);
                }
            }
            None => self.value_low = Some(value),
        }
    }
}

fn decode_input(input: &str) -> (HashMap<usize, Bot>, HashMap<usize, u8>) {
    let re_bot = Regex::new(r"^bot (?P<id>\d+) gives low to (?P<destination_low>bot|output) (?P<id_gives_low_to>\d+) and high to (?P<destination_high>bot|output) (?P<id_gives_high_to>\d+)$").unwrap();
    let re_value = Regex::new(r"^value (?P<value>\d+) goes to bot (?P<id>\d+)$").unwrap();
    let mut bots = HashMap::new();
    let mut outputs = HashMap::new();

    for line in input.lines() {
        if let Some(captures) = re_bot.captures(&line) {
            let id = captures.name("id").unwrap().as_str().parse().unwrap();
            let destination_low = captures.name("destination_low").unwrap().as_str();
            let id_gives_low_to = captures.name("id_gives_low_to").unwrap().as_str().parse().unwrap();
            let destination_high = captures.name("destination_high").unwrap().as_str();
            let id_gives_high_to = captures.name("id_gives_high_to").unwrap().as_str().parse().unwrap();

            if destination_low == "output" {
                outputs.entry(id_gives_low_to).or_insert(0);
            }

            if destination_high == "output" {
                outputs.entry(id_gives_high_to).or_insert(0);
            }

            bots.entry(id)
                    .and_modify(|mut bot: &mut Bot| {
                        bot.bot_id_gives_low_to = if destination_low == "bot" { Some(id_gives_low_to) } else { None };
                        bot.output_id_gives_low_to = if destination_low == "output" { Some(id_gives_low_to) } else { None };
                        bot.bot_id_gives_high_to = if destination_high == "bot" { Some(id_gives_high_to) } else { None };
                        bot.output_id_gives_high_to = if destination_high == "output" { Some(id_gives_high_to) } else { None };
                    })
                    .or_insert(Bot {
                        value_low: None,
                        value_high: None,
                        bot_id_gives_low_to: if destination_low == "bot" { Some(id_gives_low_to) } else { None },
                        output_id_gives_low_to: if destination_low == "output" { Some(id_gives_low_to) } else { None },
                        bot_id_gives_high_to: if destination_high == "bot" { Some(id_gives_high_to) } else { None },
                        output_id_gives_high_to: if destination_high == "output" { Some(id_gives_high_to) } else { None },
                    });
        } else if let Some(captures) = re_value.captures(&line) {
            let value = captures.name("value").unwrap().as_str().parse().unwrap();
            let id = captures.name("id").unwrap().as_str().parse().unwrap();

            bots.entry(id)
                    .and_modify(|bot| bot.attribute_value(value))
                    .or_insert(Bot {
                        value_low: Some(value),
                        value_high: None,
                        bot_id_gives_low_to: None,
                        output_id_gives_low_to: None,
                        bot_id_gives_high_to: None,
                        output_id_gives_high_to: None,
                    });
        } else {
            panic!();
        }
    }

    (bots, outputs)
}

mod part_1 {
    use crate::day_10::decode_input;

    pub fn solve(input: &str, value_low: u8, value_high: u8) -> usize {
        let (mut bots, mut outputs) = decode_input(&input);

        loop {
            let mut bots_value_attributions = Vec::new();
            let mut outputs_value_attributions = Vec::new();

            for (id, bot) in bots.iter_mut() {
                if bot.value_low == Some(value_low)
                        && bot.value_high == Some(value_high) {
                    return *id;
                }

                if bot.value_low.is_none()
                        || bot.value_high.is_none() {
                    continue;
                }

                match bot.bot_id_gives_low_to {
                    Some(id) => bots_value_attributions.push((id, bot.value_low.take().unwrap())),
                    None => outputs_value_attributions.push((bot.output_id_gives_low_to.unwrap(), bot.value_low.take().unwrap())),
                }

                match bot.bot_id_gives_high_to {
                    Some(id) => bots_value_attributions.push((id, bot.value_high.take().unwrap())),
                    None => outputs_value_attributions.push((bot.output_id_gives_high_to.unwrap(), bot.value_high.take().unwrap())),
                }
            }

            for (id, value) in bots_value_attributions {
                bots.get_mut(&id).unwrap().attribute_value(value);
            }

            for (id, value) in outputs_value_attributions {
                *outputs.get_mut(&id).unwrap() = value;
            }
        }
    }

    #[cfg(test)]
    #[test]
    fn test_1() {
        let input = "value 5 goes to bot 2
bot 2 gives low to bot 1 and high to bot 0
value 3 goes to bot 1
bot 1 gives low to output 1 and high to bot 0
bot 0 gives low to output 2 and high to output 0
value 2 goes to bot 2";

        assert_eq!(solve(&input, 2, 5), 2);
    }
}

mod part_2 {
    use crate::day_10::decode_input;

    pub fn solve(input: &str) -> usize {
        let (mut bots, mut outputs) = decode_input(&input);

        loop {
            let mut bots_value_attributions = Vec::new();
            let mut outputs_value_attributions = Vec::new();

            for (_, bot) in bots.iter_mut() {
                if bot.value_low.is_none()
                        || bot.value_high.is_none() {
                    continue;
                }

                match bot.bot_id_gives_low_to {
                    Some(id) => bots_value_attributions.push((id, bot.value_low.take().unwrap())),
                    None => outputs_value_attributions.push((bot.output_id_gives_low_to.unwrap(), bot.value_low.take().unwrap())),
                }

                match bot.bot_id_gives_high_to {
                    Some(id) => bots_value_attributions.push((id, bot.value_high.take().unwrap())),
                    None => outputs_value_attributions.push((bot.output_id_gives_high_to.unwrap(), bot.value_high.take().unwrap())),
                }
            }

            for (id, value) in bots_value_attributions {
                bots.get_mut(&id).unwrap().attribute_value(value);
            }

            for (id, value) in outputs_value_attributions {
                *outputs.get_mut(&id).unwrap() = value;
            }

            if bots.iter().all(|(_, bot)| bot.value_low.is_none() || bot.value_high.is_none()) {
                return *outputs.get(&0).unwrap() as usize
                        * *outputs.get(&1).unwrap() as usize
                        * *outputs.get(&2).unwrap() as usize
            }
        }
    }
}
