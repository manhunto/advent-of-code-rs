use std::ops::{Add, Div, Mul, Sub};
use itertools::Itertools;
use crate::direction::Direction;
use crate::point::Point;
use crate::shoelace_formula::shoelace_formula;
use crate::solutions::Solution;

pub struct Day18;

impl Solution for Day18 {
    fn part_one(&self, input: &str) -> String {
        let instructions: Vec<Instruction> = Self::parse_input(input);

        let mut last = Trench::without_color(Point::new(0, 0));
        let mut trenches: Vec<Trench> = vec![last.clone()];

        for instruction in instructions {
            for _ in 0..instruction.length {
                let new = Trench::with_color(
                    last.position.move_in(instruction.direction),
                    instruction.color.clone(),
                );

                trenches.push(new.clone());
                last = new;
            }
        }

        let points: Vec<Point> = trenches.iter().map(|t| t.position).collect();

        shoelace_formula(&points)
            .mul(2)
            .sub(points.len() as i32)
            .div(2)
            .add(1)
            .add(points.len() as i32)
            .to_string()
    }

    fn part_two(&self, input: &str) -> String {
        String::from('0')
    }
}

impl Day18 {
    fn parse_input(input: &str) -> Vec<Instruction> {
        input
            .lines()
            .map(|line| {
                let (dir, length, color) = line.split_whitespace().collect_tuple().unwrap();

                let direction = match dir {
                    "R" => Direction::East,
                    "L" => Direction::West,
                    "U" => Direction::North,
                    "D" => Direction::South,
                    _ => unreachable!()
                };

                Instruction {
                    direction,
                    length: length.parse().unwrap(),
                    color: color.trim_matches(&['(', ')', '#'] as &[_]).to_string(),
                }
            })
            .collect()
    }
}

#[derive(Debug)]
struct Instruction {
    direction: Direction,
    length: u8,
    color: String,
}

#[derive(Debug, Clone)]
struct Trench {
    position: Point,
    color: Option<String>,
}

impl Trench {
    fn without_color(position: Point) -> Self {
        Self { position, color: None }
    }

    fn with_color(position: Point, color: String) -> Self {
        Self { position, color: Some(color) }
    }
}

#[cfg(test)]
mod tests {
    use crate::file_system::read_example;
    use crate::solutions::day18::Day18;
    use crate::solutions::Solution;

    #[test]
    fn part_one_example_test() {
        let input = read_example("18");

        assert_eq!("62", Day18.part_one(&input.as_str()));
    }
}
