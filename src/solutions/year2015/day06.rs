use crate::solutions::Solution;
use crate::utils::grid::Grid;
use crate::utils::point::Point;
use crate::utils::surface_range::SurfaceRange;
use std::fmt::Debug;

pub struct Day06;

impl Solution for Day06 {
    fn part_one(&self, input: &str) -> String {
        let mut grid = Grid::filled(
            SurfaceRange::from((Point::new(0, 0), Point::new(999, 999))),
            0,
        );
        let instructions = self.parse(input);

        for instruction in instructions {
            instruction.apply_part_one(&mut grid);
        }

        grid.get_all_positions(&1).len().to_string()
    }

    fn part_two(&self, input: &str) -> String {
        let mut grid = Grid::filled(
            SurfaceRange::from((Point::new(0, 0), Point::new(999, 999))),
            0,
        );
        let instructions = self.parse(input);

        for instruction in instructions {
            instruction.apply_part_two(&mut grid);
        }

        grid.all().values().sum::<u64>().to_string()
    }
}

impl Day06 {
    fn parse(&self, input: &str) -> Vec<Box<dyn Instruction>> {
        input
            .lines()
            .map(|line| -> Box<dyn Instruction> {
                let parts = line.split_whitespace().collect::<Vec<_>>();

                if parts[0] == "turn" && parts[1] == "on" {
                    Box::new(TurnOn::from(self.parse_points(parts[2], parts[4])))
                } else if parts[0] == "turn" && parts[1] == "off" {
                    Box::new(TurnOff::from(self.parse_points(parts[2], parts[4])))
                } else if parts[0] == "toggle" {
                    Box::new(Toggle::from(self.parse_points(parts[1], parts[3])))
                } else {
                    unreachable!()
                }
            })
            .collect()
    }

    fn parse_points(&self, from_str: &str, to_str: &str) -> (Point, Point) {
        (from_str.parse().unwrap(), to_str.parse().unwrap())
    }
}

trait Instruction: Debug {
    fn apply_part_one(&self, grid: &mut Grid<u64>);
    fn apply_part_two(&self, grid: &mut Grid<u64>);
}

#[derive(Debug)]
struct TurnOn {
    surface_range: SurfaceRange,
}

impl From<(Point, Point)> for TurnOn {
    fn from(value: (Point, Point)) -> Self {
        Self {
            surface_range: SurfaceRange::from(value),
        }
    }
}

impl Instruction for TurnOn {
    fn apply_part_one(&self, grid: &mut Grid<u64>) {
        grid.modify_many(self.surface_range.points(), 1)
    }

    fn apply_part_two(&self, grid: &mut Grid<u64>) {
        grid.modify_many_with(self.surface_range.points(), |b| *b += 1)
    }
}

#[derive(Debug)]
struct TurnOff {
    surface_range: SurfaceRange,
}

impl From<(Point, Point)> for TurnOff {
    fn from(value: (Point, Point)) -> Self {
        Self {
            surface_range: SurfaceRange::from(value),
        }
    }
}

impl Instruction for TurnOff {
    fn apply_part_one(&self, grid: &mut Grid<u64>) {
        grid.modify_many(self.surface_range.points(), 0)
    }

    fn apply_part_two(&self, grid: &mut Grid<u64>) {
        grid.modify_many_with(self.surface_range.points(), |b| *b = u64::max(*b - 1, 0))
    }
}

#[derive(Debug)]
struct Toggle {
    surface_range: SurfaceRange,
}

impl From<(Point, Point)> for Toggle {
    fn from(value: (Point, Point)) -> Self {
        Self {
            surface_range: SurfaceRange::from(value),
        }
    }
}

impl Instruction for Toggle {
    fn apply_part_one(&self, grid: &mut Grid<u64>) {
        grid.modify_many_with(self.surface_range.points(), |b| {
            *b = if *b == 0 { 1 } else { 0 }
        })
    }

    fn apply_part_two(&self, grid: &mut Grid<u64>) {
        grid.modify_many_with(self.surface_range.points(), |b| *b += 2)
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
    }
}
