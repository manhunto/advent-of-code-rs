use crate::solutions::year2015::day06::InstructionType::{Toggle, TurnOff, TurnOn};
use crate::solutions::Solution;
use crate::utils::point::Point;
use crate::utils::surface_range::SurfaceRange;
use std::collections::HashMap;
use std::fmt::Debug;

pub struct Day06;

impl Solution for Day06 {
    fn part_one(&self, input: &str) -> String {
        let apply =
            |instruction: &Instruction, grid: &mut LightGrid| instruction.apply_part_one(grid);

        self.apply_instructions(input, apply)
            .grid
            .values()
            .filter(|v| **v == 1)
            .count()
            .to_string()
    }

    fn part_two(&self, input: &str) -> String {
        let apply =
            |instruction: &Instruction, grid: &mut LightGrid| instruction.apply_part_two(grid);

        self.apply_instructions(input, apply)
            .grid
            .values()
            .sum::<u64>()
            .to_string()
    }
}

impl Day06 {
    fn parse(&self, input: &str) -> Vec<Instruction> {
        input
            .lines()
            .map(|line| {
                let parts = line.split_whitespace().collect::<Vec<_>>();

                if parts[0] == "turn" && parts[1] == "on" {
                    Instruction::new(TurnOn, self.parse_points(parts[2], parts[4]))
                } else if parts[0] == "turn" && parts[1] == "off" {
                    Instruction::new(TurnOff, self.parse_points(parts[2], parts[4]))
                } else if parts[0] == "toggle" {
                    Instruction::new(Toggle, self.parse_points(parts[1], parts[3]))
                } else {
                    unreachable!()
                }
            })
            .collect()
    }

    fn parse_points(&self, from_str: &str, to_str: &str) -> (Point, Point) {
        (from_str.parse().unwrap(), to_str.parse().unwrap())
    }

    fn apply_instructions<F>(&self, input: &str, mut func: F) -> LightGrid
    where
        F: FnMut(&Instruction, &mut LightGrid),
    {
        let mut grid = LightGrid::default();

        for instruction in self.parse(input) {
            func(&instruction, &mut grid);
        }

        grid
    }
}

#[derive(Default)]
struct LightGrid {
    grid: HashMap<Point, u64>,
}

#[derive(Debug)]
enum InstructionType {
    TurnOn,
    TurnOff,
    Toggle,
}

#[derive(Debug)]
struct Instruction {
    instruction_type: InstructionType,
    surface_range: SurfaceRange,
}

impl Instruction {
    fn new(instruction_type: InstructionType, points: (Point, Point)) -> Self {
        Self {
            instruction_type,
            surface_range: SurfaceRange::from(points),
        }
    }

    fn apply_part_one(&self, grid: &mut LightGrid) {
        for point in self.surface_range.points() {
            match self.instruction_type {
                TurnOn => {
                    *grid.grid.entry(point).or_default() = 1;
                }
                TurnOff => {
                    *grid.grid.entry(point).or_default() = 0;
                }
                Toggle => {
                    grid.grid
                        .entry(point)
                        .and_modify(|v| *v = if *v == 0 { 1 } else { 0 })
                        .or_insert(1);
                }
            }
        }
    }

    fn apply_part_two(&self, grid: &mut LightGrid) {
        for point in self.surface_range.points() {
            match self.instruction_type {
                TurnOn => {
                    *grid.grid.entry(point).or_default() += 1;
                }
                TurnOff => {
                    grid.grid
                        .entry(point)
                        .and_modify(|v| *v = v.saturating_sub(1))
                        .or_default();
                }
                Toggle => {
                    *grid.grid.entry(point).or_default() += 2;
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_one_example_test() {
        assert_eq!("1000000", Day06.part_one("turn on 0,0 through 999,999"));
        assert_eq!("1000", Day06.part_one("toggle 0,0 through 999,0"));
    }

    #[test]
    fn part_two_example_test() {
        assert_eq!("1", Day06.part_two("turn on 0,0 through 0,0"));
        assert_eq!("2000000", Day06.part_two("toggle 0,0 through 999,999"));
        assert_eq!(
            "7",
            Day06.part_two(
                r#"toggle 0,0 through 0,3
        turn off 0,0 through 0,0"#
            )
        );
        assert_eq!(
            "6",
            Day06.part_two(
                r#"toggle 0,0 through 0,3
        turn off 0,0 through 0,0
        turn off 0,0 through 0,0"#
            )
        );
        assert_eq!(
            "6",
            Day06.part_two(
                r#"toggle 0,0 through 0,3
        turn off 0,0 through 0,0
        turn off 0,0 through 0,0
        turn off 0,0 through 0,0"#
            )
        );
    }
}
