use crate::solutions::Solution;
use std::collections::HashMap;

type BotsHashMap = HashMap<usize, Bot>;
type InstructionsHashMap = HashMap<usize, Decision>;
type OutputsHashMap = HashMap<usize, usize>;

pub struct Day10 {
    lower: usize,
    higher: usize,
}

impl Solution for Day10 {
    fn part_one(&self, input: &str) -> String {
        let (instructions, mut bots) = self.parse(input);

        loop {
            let (bot_id, bot) = bots.next_for_handover().unwrap();
            let (lower, higher) = bot.handover();

            if lower == self.lower && higher == self.higher {
                return bot_id.to_string();
            }

            let decision = instructions.for_bot(bot_id);
            bots.decision(bot_id, decision);
        }
    }

    fn part_two(&self, input: &str) -> String {
        let (instructions, mut bots) = self.parse(input);

        while let Some((bot_id, _)) = bots.next_for_handover() {
            let decision = instructions.for_bot(bot_id);
            bots.decision(bot_id, decision);
        }

        bots.output_values_in_0_1_2()
            .iter()
            .product::<usize>()
            .to_string()
    }
}

impl Default for Day10 {
    fn default() -> Self {
        Self {
            lower: 17,
            higher: 61,
        }
    }
}

impl Day10 {
    fn parse(&self, input: &str) -> (Instructions, BotsAndOutputs) {
        let mut bot_list = BotsAndOutputs::new();
        let mut bot_decisions: InstructionsHashMap = HashMap::new();

        input.lines().for_each(|line| {
            let parts: Vec<&str> = line.split_whitespace().collect();

            match parts.as_slice() {
                // value 5 goes to bot 2
                ["value", value, "goes", "to", "bot", bot] => {
                    let value = value.parse::<usize>().unwrap();
                    let bot_id = bot.parse::<usize>().unwrap();

                    bot_list.handover(bot_id, value);
                }
                // bot 2 gives low to bot 1 and high to bot 0
                ["bot", bot, "gives", "low", "to", low_type, low_value, "and", "high", "to", high_type, high_value] => {
                    let bot_id = bot.parse().unwrap();
                    let lower = Handover::from((*low_type, *low_value));
                    let higher = Handover::from((*high_type, *high_value));

                    bot_decisions.insert(bot_id, Decision::new(lower, higher));
                }
                _ => unreachable!()
            }
        });

        (Instructions::new(bot_decisions), bot_list)
    }
}

#[derive(Debug)]
struct Bot {
    microchips: [Option<usize>; 2],
}

impl Bot {
    fn new(value: usize) -> Self {
        Self {
            microchips: [Some(value), None],
        }
    }

    #[inline]
    fn can_handover(&self) -> bool {
        self.microchips[0].is_some() && self.microchips[1].is_some()
    }

    fn handover(&self) -> (usize, usize) {
        let mut microchips = self.microchips;
        microchips.sort();

        (microchips[0].unwrap(), microchips[1].unwrap())
    }

    fn add(&mut self, value: usize) -> Result<(), &'static str> {
        if self.microchips[1].is_some() {
            return Err("too many chips");
        }

        self.microchips[1] = Some(value);

        Ok(())
    }
}

#[derive(Debug)]
struct BotsAndOutputs {
    bots: BotsHashMap,
    outputs: OutputsHashMap,
}

impl BotsAndOutputs {
    fn new() -> Self {
        Self {
            bots: BotsHashMap::new(),
            outputs: HashMap::new(),
        }
    }

    fn next_for_handover(&self) -> Option<(usize, &Bot)> {
        self.bots
            .iter()
            .find(|(_, bot)| bot.can_handover())
            .map(|(id, bot)| (*id, bot))
    }

    fn handover(&mut self, bot_id: usize, value: usize) {
        self.bots
            .entry(bot_id)
            .and_modify(|bot| {
                bot.add(value).unwrap();
            })
            .or_insert(Bot::new(value));
    }

    fn add_output(&mut self, output: usize, value: usize) {
        self.outputs.insert(output, value);
    }

    fn output_values_in_0_1_2(&self) -> Vec<usize> {
        self.outputs
            .iter()
            .filter(|(&output, _)| output <= 2)
            .map(|(_, v)| *v)
            .collect()
    }

    fn decision(&mut self, bot_id: usize, decision: &Decision) {
        let bot = self.bots.remove(&bot_id).unwrap();
        let (lower, higher) = bot.handover();

        match decision.lower {
            Handover::Bot(lower_bot_id) => self.handover(lower_bot_id, lower),
            Handover::Output(output) => self.add_output(output, lower),
        };

        match decision.higher {
            Handover::Bot(higher_bot_id) => self.handover(higher_bot_id, higher),
            Handover::Output(output) => self.add_output(output, higher),
        };
    }
}

#[derive(Debug)]
struct Instructions {
    bot_decision: InstructionsHashMap,
}

impl Instructions {
    fn new(bot_decision: InstructionsHashMap) -> Self {
        Self { bot_decision }
    }

    fn for_bot(&self, bot_id: usize) -> &Decision {
        self.bot_decision.get(&bot_id).unwrap()
    }
}

#[derive(Debug)]
struct Decision {
    lower: Handover,
    higher: Handover,
}

impl Decision {
    fn new(lower: Handover, higher: Handover) -> Self {
        Self { lower, higher }
    }
}

#[derive(Debug)]
enum Handover {
    Bot(usize),
    #[allow(dead_code)]
    Output(usize),
}

impl From<(&str, &str)> for Handover {
    fn from((t, v): (&str, &str)) -> Self {
        let value = v.parse::<usize>().unwrap();

        match t {
            "bot" => Handover::Bot(value),
            "output" => Handover::Output(value),
            _ => unreachable!(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = r#"value 5 goes to bot 2
bot 2 gives low to bot 1 and high to bot 0
value 3 goes to bot 1
bot 1 gives low to output 1 and high to bot 0
bot 0 gives low to output 2 and high to output 0
value 2 goes to bot 2"#;

    #[test]
    fn part_one_example() {
        assert_eq!("2", day().part_one(EXAMPLE));
    }

    #[test]
    fn part_two_example() {
        assert_eq!("30", day().part_two(EXAMPLE));
    }

    fn day() -> Day10 {
        Day10 {
            lower: 2,
            higher: 5,
        }
    }
}
