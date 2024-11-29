use crate::solutions::Solution;
use regex::Regex;
use std::collections::{HashMap, HashSet};
use std::str;

pub struct Day04;

impl Solution for Day04 {
    fn part_one(&self, input: &str) -> String {
        input
            .lines()
            .map(|line| {
                let how_many_winning = parse_line(line).how_many_winning();

                if how_many_winning == 0 {
                    return 0;
                }

                2u32.pow(how_many_winning - 1)
            })
            .sum::<u32>()
            .to_string()
    }

    fn part_two(&self, input: &str) -> String {
        let mut scratchards: HashMap<i32, u32> = HashMap::new();

        input
            .lines()
            .map(|line| {
                let card = parse_line(line);
                let how_many_winning = card.how_many_winning();
                let amount_of_current_card = scratchards.get(&card.id).unwrap_or(&0) + 1;

                if how_many_winning > 0 {
                    let from = card.id + 1;
                    let to = card.id + how_many_winning as i32;

                    for winning_card_id in from..to + 1 {
                        *scratchards.entry(winning_card_id).or_insert(0) += amount_of_current_card;
                    }
                }

                amount_of_current_card
            })
            .sum::<u32>()
            .to_string()
    }
}

fn parse_line(line: &str) -> Card {
    let re_all = Regex::new(r"Card\s+(\d+):(.*\d+.*)\|(.*\d+.*)").unwrap();

    let captures = re_all.captures(line).unwrap();

    let card_id = captures.get(1).unwrap().as_str();
    let winning_numbers_part = captures.get(2).unwrap().as_str();
    let your_numbers_part = captures.get(3).unwrap().as_str();

    let re_only_numbers = Regex::new(r"\d+").unwrap();

    let winning_numbers: HashSet<i32> = re_only_numbers
        .find_iter(winning_numbers_part)
        .map(|m| m.as_str().parse().unwrap())
        .collect();

    let your_numbers: HashSet<i32> = re_only_numbers
        .find_iter(your_numbers_part)
        .map(|m| m.as_str().parse().unwrap())
        .collect();

    Card {
        id: card_id.parse().unwrap(),
        winning_numbers,
        your_numbers,
    }
}

#[derive(PartialEq, Debug)]
struct Card {
    id: i32,
    winning_numbers: HashSet<i32>,
    your_numbers: HashSet<i32>,
}

impl Card {
    fn how_many_winning(&self) -> u32 {
        self.winning_numbers
            .intersection(&self.your_numbers)
            .count() as u32
    }
}

#[cfg(test)]
mod tests {
    use crate::file_system::read_2023_example;
    use crate::solutions::year2023::day04::{parse_line, Card, Day04};
    use crate::solutions::Solution;

    #[test]
    fn part_one_example_test() {
        let input = read_2023_example("04");

        assert_eq!("13", Day04.part_one(input.as_str()));
    }

    #[test]
    fn part_two_example_test() {
        let input = read_2023_example("04");

        assert_eq!("30", Day04.part_two(input.as_str()));
    }

    #[test]
    fn parse_line_test() {
        assert_eq!(
            Card {
                id: 1,
                winning_numbers: vec![41, 48, 83, 86, 17].into_iter().collect(),
                your_numbers: vec![83, 86, 6, 31, 17, 9, 48, 53].into_iter().collect(),
            },
            parse_line("Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53")
        );

        assert_eq!(
            Card {
                id: 6,
                winning_numbers: vec![1, 18, 3, 56, 72].into_iter().collect(),
                your_numbers: vec![74, 77, 10, 23, 35, 67, 36, 11].into_iter().collect(),
            },
            parse_line("Card   6:  1 18  3 56 72 | 74 77 10 23 35 67 36 11")
        );
    }
}
