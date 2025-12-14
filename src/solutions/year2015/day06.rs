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
            false,
        );
        let instructions = self.parse(input);

        for instruction in instructions {
            instruction.apply(&mut grid);
        }

        grid.get_all_positions(&true).len().to_string()
    }

    fn part_two(&self, _input: &str) -> String {
        String::from("0")
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
    fn apply(&self, grid: &mut Grid<bool>);
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
    fn apply(&self, grid: &mut Grid<bool>) {
        grid.modify_many(self.surface_range.points(), true)
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
    fn apply(&self, grid: &mut Grid<bool>) {
        grid.modify_many(self.surface_range.points(), false)
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
    fn apply(&self, grid: &mut Grid<bool>) {
        grid.modify_many_with(self.surface_range.points(), |b| *b = !*b)
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
}
