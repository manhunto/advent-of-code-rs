use crate::solutions::Solution;
use std::iter::Sum;
use std::num::ParseIntError;
use std::ops::{Add, Mul};
use std::str::FromStr;

const MAX_TEASPOONS: usize = 100;

pub struct Day15;

impl Solution for Day15 {
    fn part_one(&self, input: &str) -> String {
        let ingredients = self.parse(input);
        let combinations = self.generate_combinations_optimized(&ingredients);

        let mut best: i64 = 0;

        for combination in combinations {
            let score = self.calculate_score(combination, &ingredients);

            if score > best {
                best = score;
            }
        }

        best.to_string()
    }

    fn part_two(&self, _input: &str) -> String {
        String::from("0")
    }
}

impl Day15 {
    fn parse(&self, input: &str) -> Vec<Ingredient> {
        input.lines().map(|line| line.parse().unwrap()).collect()
    }

    fn generate_combinations_optimized(&self, ingredients: &[Ingredient]) -> Vec<Vec<usize>> {
        let mut combinations: Vec<Vec<usize>> = Vec::new();
        let mut current: Vec<usize> = Vec::new();

        self.sub_generate_combinations_optimized(
            MAX_TEASPOONS,
            ingredients.len(),
            &mut current,
            &mut combinations,
        );

        combinations
    }

    fn sub_generate_combinations_optimized(
        &self,
        total: usize,
        num_ingredients: usize,
        current: &mut Vec<usize>,
        results: &mut Vec<Vec<usize>>,
    ) {
        if num_ingredients == 1 {
            current.push(total);
            results.push(current.clone());
            current.pop();
            return;
        }

        for amount in 0..=total {
            current.push(amount);
            self.sub_generate_combinations_optimized(
                total - amount,
                num_ingredients - 1,
                current,
                results,
            );
            current.pop();
        }
    }

    fn calculate_score(&self, combination: Vec<usize>, ingredients: &[Ingredient]) -> i64 {
        combination
            .iter()
            .enumerate()
            .map(|(i, teaspoons)| ingredients[i] * *teaspoons as i64)
            .sum::<Ingredient>()
            .score()
    }
}

#[derive(Debug, Copy, Clone, Default)]
struct Ingredient {
    capacity: i64,
    durability: i64,
    flavor: i64,
    texture: i64,
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
        })
    }
}

impl Ingredient {
    fn score(&self) -> i64 {
        self.capacity.max(0) * self.durability.max(0) * self.flavor.max(0) * self.texture.max(0)
    }
}

impl Mul<i64> for Ingredient {
    type Output = Self;

    fn mul(self, rhs: i64) -> Self::Output {
        Self {
            capacity: self.capacity * rhs,
            durability: self.durability * rhs,
            flavor: self.flavor * rhs,
            texture: self.texture * rhs,
        }
    }
}

impl Add for Ingredient {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            capacity: self.capacity + rhs.capacity,
            durability: self.durability + rhs.durability,
            flavor: self.flavor + rhs.flavor,
            texture: self.texture + rhs.texture,
        }
    }
}

impl Sum<Ingredient> for Ingredient {
    fn sum<I: Iterator<Item = Ingredient>>(iter: I) -> Self {
        let mut total: Ingredient = Ingredient::default();
        for ingredient in iter {
            total = total + ingredient;
        }

        total
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

    #[test]
    fn calculate_score() {
        let ingredients = Day15.parse(EXAMPLE);
        let score = Day15.calculate_score(vec![44, 56], &ingredients);

        assert_eq!(62842880, score);
    }
}
