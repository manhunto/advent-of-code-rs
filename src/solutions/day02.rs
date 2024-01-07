use crate::solutions::Solution;
use itertools::Itertools;
use std::cmp::max;

pub struct Day02;

impl Solution for Day02 {
    fn part_one(&self, input: &str) -> String {
        const POSSIBLE_RED: i32 = 12;
        const POSSIBLE_GREEN: i32 = 13;
        const POSSIBLE_BLUE: i32 = 14;

        input
            .lines()
            .map(parse_line)
            .filter(|game| !game.is_impossible(POSSIBLE_RED, POSSIBLE_GREEN, POSSIBLE_BLUE))
            .map(|game: Game| game.id)
            .sum::<i32>()
            .to_string()
    }

    fn part_two(&self, input: &str) -> String {
        input
            .lines()
            .map(parse_line)
            .map(|game: Game| game.get_min_balls_product())
            .sum::<i32>()
            .to_string()
    }
}

#[derive(PartialEq, Debug)]
struct Game {
    pub id: i32,
    pub sets: Vec<Set>,
}

impl Game {
    fn is_impossible(&self, red: i32, green: i32, blue: i32) -> bool {
        self.sets
            .iter()
            .any(|set| set.red > red || set.green > green || set.blue > blue)
    }

    fn get_min_balls_product(&self) -> i32 {
        let mut min_red = 0;
        let mut min_green = 0;
        let mut min_blue = 0;

        for set in &self.sets {
            min_red = max(min_red, set.red);
            min_green = max(min_green, set.green);
            min_blue = max(min_blue, set.blue);
        }

        min_red * min_green * min_blue
    }
}

#[derive(PartialEq, Debug)]
struct Set {
    pub red: i32,
    pub green: i32,
    pub blue: i32,
}

fn parse_line(input: &str) -> Game {
    let after_split: Vec<&str> = input.split(": ").collect();
    let id: i32 = after_split[0].replace("Game ", "").parse().unwrap();

    let set_strings: Vec<&str> = after_split[1].split("; ").collect();

    let sets: Vec<Set> = set_strings
        .iter()
        .map(|line| {
            let split: Vec<&str> = line.split_terminator(", ").collect();

            let mut red = 0;
            let mut green = 0;
            let mut blue = 0;

            for s in split {
                let (value, color) = s.split(' ').collect_tuple().unwrap();

                match color {
                    "red" => red = value.parse().unwrap(),
                    "green" => green = value.parse().unwrap(),
                    "blue" => blue = value.parse().unwrap(),
                    _ => unreachable!(),
                }
            }

            Set { red, green, blue }
        })
        .collect();

    Game { id, sets }
}

#[cfg(test)]
mod tests {
    use crate::file_system::read_example;
    use crate::solutions::day02::{parse_line, Day02, Game, Set};
    use crate::solutions::Solution;

    #[test]
    fn part_one_example_test() {
        let input = read_example("02");

        assert_eq!("8", Day02.part_one(input.as_str()));
    }

    #[test]
    fn part_two_example_test() {
        let input = read_example("02");

        assert_eq!("2286", Day02.part_two(input.as_str()));
    }

    #[test]
    fn parse_line_test() {
        let line = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green";
        assert_eq!(
            Game {
                id: 1,
                sets: vec![
                    Set {
                        red: 4,
                        green: 0,
                        blue: 3
                    },
                    Set {
                        red: 1,
                        green: 2,
                        blue: 6
                    },
                    Set {
                        red: 0,
                        green: 2,
                        blue: 0
                    },
                ],
            },
            parse_line(line)
        )
    }
}
