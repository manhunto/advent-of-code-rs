use std::collections::HashSet;
use crate::solutions::Solution;
use std::str;
use regex::Regex;

pub struct Day04;

impl Solution for Day04 {
    fn part_one(&self, input: &str) -> String {
        input
            .lines()
            .map(|line| {
                let result = parse_line(&line).how_many_winning();

                if result == 0 {
                    return 0;
                }

                return u32::pow(2, result - 1);
            })
            .sum::<u32>()
            .to_string()
    }

    fn part_two(&self, input: &str) -> String {
        String::from("0")
    }
}

fn parse_line(line: &str) -> Card{
    let re_all = Regex::new(r"Card\s+(\d+):(.*\d+.*)\|(.*\d+.*)").unwrap();

    let captures = re_all.captures(line).unwrap();

    let card_id = captures.get(1).unwrap().as_str();
    let winning_numbers_part = captures.get(2).unwrap().as_str();
    let your_numbers_part = captures.get(3).unwrap().as_str();

    let re_only_numbers = Regex::new(r"\d+").unwrap();

    let winning_numbers: Vec<i32> = re_only_numbers
        .find_iter(winning_numbers_part)
        .map(|m| m.as_str().parse().unwrap())
        .collect();

    let your_numbers: Vec<i32> = re_only_numbers
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
    winning_numbers: Vec<i32>,
    your_numbers: Vec<i32>
}

impl Card {
    fn how_many_winning(&self) -> u32 {
        let winning_set: HashSet<i32> = self.winning_numbers.clone().into_iter().collect();
        let your_set: HashSet<i32> = self.your_numbers.clone().into_iter().collect();

        winning_set.intersection(&your_set).count() as u32
    }
}

#[cfg(test)]
mod tests {
    use crate::file_system::read_example;
    use crate::solutions::day04::{Card, Day04, parse_line};
    use crate::solutions::Solution;

    #[test]
    fn part_one_example_test() {
        let input = read_example("04");

        assert_eq!("13", Day04.part_one(&input.as_str()));
    }

    #[test]
    fn parse_line_test() {
        assert_eq!(Card {
            id: 1,
            winning_numbers: vec![41, 48, 83, 86, 17],
            your_numbers: vec![83, 86, 6, 31, 17, 9, 48, 53],
        }, parse_line("Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53"));

        assert_eq!(Card {
            id: 6,
            winning_numbers: vec![1, 18, 3, 56, 72],
            your_numbers: vec![74, 77, 10, 23, 35, 67, 36, 11],
        }, parse_line("Card   6:  1 18  3 56 72 | 74 77 10 23 35 67 36 11"));
    }
}
