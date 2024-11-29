use crate::direction::Direction;
use crate::point::Point;
use crate::shoelace_formula::shoelace_formula;
use crate::solutions::Solution;
use itertools::Itertools;

pub struct Day18;

impl Solution for Day18 {
    fn part_one(&self, input: &str) -> String {
        let instructions: Vec<Instruction> = Self::parse_input_part_one(input);

        Self::solve(instructions)
    }

    fn part_two(&self, input: &str) -> String {
        let instructions: Vec<Instruction> = Self::parse_input_part_two(input);

        Self::solve(instructions)
    }
}

impl Day18 {
    fn parse_input_part_one(input: &str) -> Vec<Instruction> {
        input
            .lines()
            .map(|line| {
                let (dir, length, _) = line.split_whitespace().collect_tuple().unwrap();

                let direction = match dir {
                    "R" => Direction::East,
                    "L" => Direction::West,
                    "U" => Direction::North,
                    "D" => Direction::South,
                    _ => unreachable!(),
                };

                Instruction::new(direction, length.parse().unwrap())
            })
            .collect()
    }

    fn parse_input_part_two(input: &str) -> Vec<Instruction> {
        input
            .lines()
            .map(|line| {
                let (_, _, color) = line.split_whitespace().collect_tuple().unwrap();

                let dir: u8 = color[7..8].parse().unwrap();
                let length = usize::from_str_radix(&color[2..7], 16).unwrap();

                let direction = match dir {
                    0 => Direction::East,
                    1 => Direction::South,
                    2 => Direction::West,
                    3 => Direction::North,
                    _ => unreachable!(),
                };

                Instruction::new(direction, length)
            })
            .collect()
    }

    fn solve(instructions: Vec<Instruction>) -> String {
        let mut last = Point::new(0, 0);
        let mut trenches: Vec<Point> = vec![last];

        for instruction in instructions {
            let new = last.move_in_with_length(instruction.direction, instruction.length as isize);

            trenches.push(new);
            last = new;
        }

        shoelace_formula(&trenches).to_string()
    }
}

#[derive(Debug)]
struct Instruction {
    direction: Direction,
    length: usize,
}

impl Instruction {
    fn new(direction: Direction, length: usize) -> Self {
        Self { direction, length }
    }
}

#[cfg(test)]
mod tests {
    use crate::file_system::read_2023_example;
    use crate::solutions::year2023::day18::Day18;
    use crate::solutions::Solution;

    #[test]
    fn part_one_example_test() {
        let input = read_2023_example("18");

        assert_eq!("62", Day18.part_one(input.as_str()));
    }

    #[test]
    fn part_two_example_test() {
        let input = read_2023_example("18");

        assert_eq!("952408144115", Day18.part_two(input.as_str()));
    }
}
