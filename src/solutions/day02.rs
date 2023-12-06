use crate::solutions::Solution;
use regex::{Regex};

pub struct Day02;

impl Solution for Day02 {
    fn part_one(&self, input: &str) -> String {
        const POSSIBLE_RED: i32 = 12;
        const POSSIBLE_GREEN: i32 = 13;
        const POSSIBLE_BLUE: i32 = 14;

        input
            .lines()
            .map(|line: &str| parse_line(line))
            .filter(|game| game.is_impossible(POSSIBLE_RED, POSSIBLE_GREEN, POSSIBLE_BLUE) == false)
            .map(|game: Game| game.id)
            .sum::<i32>()
            .to_string()
    }

    fn part_two(&self, input: &str) -> String {
        "0".to_string()
    }
}

#[derive(PartialEq, Debug)]
struct Game {
    pub id: i32,
    pub sets: Vec<Set>,
}

impl Game {
    fn is_impossible(&self, red: i32, green: i32, blue: i32) -> bool {
        for set in &self.sets {
            if set.red > red || set.green > green || set.blue > blue {
                return true;
            }
        }

        return false;
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
    let id: i32 = after_split[0]
        .replace("Game ", "")
        .parse()
        .unwrap();

    let set_strings: Vec<&str> = after_split[1].split("; ").collect();
    let sets: Vec<Set> = set_strings
        .iter()
        .map(|line| {
            Set {
                red: parse_color("red", &line),
                green: parse_color("green", &line),
                blue: parse_color("blue", &line),
            }
        })
        .collect();


    Game { id, sets }
}

fn parse_color(color: &str, line: &str) -> i32 {
    let red_regex = Regex::new(&*format!(r"(\d+) {}", color)).unwrap();

    let red = match red_regex.captures(line) {
        Some(cap) => cap.get(1).map_or("0", |x| x.as_str()),
        None => "0"
    };

    red.parse::<i32>().unwrap()
}

#[cfg(test)]
mod tests {
    use crate::file_system::read_example;
    use crate::solutions::day02::{Day02, Game, parse_line, Set};
    use crate::solutions::Solution;

    #[test]
    fn part_one_example_test() {
        let input = read_example("02");

        assert_eq!("8", Day02.part_one(&input.as_str()));
    }

    #[test]
    fn parse_line_test() {
        let line = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green";
        assert_eq!(Game {
            id: 1,
            sets: vec![
                Set { red: 4, green: 0, blue: 3 },
                Set { red: 1, green: 2, blue: 6 },
                Set { red: 0, green: 2, blue: 0 },
            ],
        }, parse_line(line))
    }
}