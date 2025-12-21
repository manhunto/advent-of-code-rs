use crate::solutions::Solution;
use std::num::ParseIntError;
use std::str::FromStr;

pub struct Day15;

impl Solution for Day15 {
    fn part_one(&self, _input: &str) -> String {
        let ingredients = self.parse(_input);

        println!("{:?}", ingredients);

        String::from("0")
    }

    fn part_two(&self, _input: &str) -> String {
        String::from("0")
    }
}

impl Day15 {
    fn parse(&self, input: &str) -> Vec<Ingredient> {
        input.lines().map(|line| line.parse().unwrap()).collect()
    }
}

#[derive(Debug)]
#[allow(dead_code)]
struct Ingredient {
    capacity: i64,
    durability: i64,
    flavor: i64,
    texture: i64,
    calories: i64,
}

impl FromStr for Ingredient {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts = s.split_whitespace().collect::<Vec<_>>();

        let parse = |str: &str| str.trim_end_matches(',').parse::<i64>();

        Ok(Self {
            capacity: parse(parts[2])?,
            durability: parse(parts[4])?,
            flavor: parse(parts[6])?,
            texture: parse(parts[8])?,
            calories: parse(parts[10])?,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = r#"Butterscotch: capacity -1, durability -2, flavor 6, texture 3, calories 8
Cinnamon: capacity 2, durability 3, flavor -2, texture -1, calories 3"#;

    #[test]
    fn part_one_example() {
        assert_eq!("62842880", Day15.part_one(EXAMPLE));
    }
}
