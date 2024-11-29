use crate::direction::Direction;
use crate::grid::Grid;
use crate::point::Point;
use crate::solutions::Solution;
use crate::utils::vector::Vector;
use itertools::Itertools;
use std::collections::VecDeque;
use Direction::{East, North, South, West};

pub struct Day16;

impl Solution for Day16 {
    fn part_one(&self, input: &str) -> String {
        let grid: Grid<Tile> = Grid::from(input);

        Self::energize(Vector::new(Point::new(0, 0), East), &grid).to_string()
    }

    fn part_two(&self, input: &str) -> String {
        let grid: Grid<Tile> = Grid::from(input);
        let starting_points: Vec<Vector> = grid.surface_range().vectors_pointing_inwards();

        starting_points
            .into_iter()
            .map(|start| Self::energize(start, &grid))
            .max()
            .unwrap()
            .to_string()
    }
}

impl Day16 {
    fn energize(start: Vector, grid: &Grid<Tile>) -> usize {
        let surface_range = grid.surface_range();

        let mut beams: VecDeque<Vector> = VecDeque::new();
        beams.push_back(start);

        let mut history: Vec<Vector> = Vec::new();

        while let Some(mut beam) = beams.pop_front() {
            while surface_range.contains(beam.position()) && !history.contains(&beam) {
                let position = beam.position();

                history.push(beam);

                let tile = grid.get_for_point(&position).unwrap();
                let facing = beam.facing();

                beam = match tile {
                    Tile::SplitterVer if [East, West].contains(&facing) => {
                        let new = beam.rotate_ccw().step();
                        beams.push_back(new);

                        beam.rotate_cw().step()
                    }
                    Tile::SplitterHor if [South, North].contains(&facing) => {
                        let new = beam.rotate_ccw().step();
                        beams.push_back(new);

                        beam.rotate_cw().step()
                    }
                    Tile::MirrorFWD if [South, North].contains(&facing) => beam.rotate_cw().step(),
                    Tile::MirrorFWD if [East, West].contains(&facing) => beam.rotate_ccw().step(),
                    Tile::MirrorBWD if [South, North].contains(&facing) => beam.rotate_ccw().step(),
                    Tile::MirrorBWD if [East, West].contains(&facing) => beam.rotate_cw().step(),
                    _ => beam.step(),
                };
            }
        }

        history
            .iter()
            .map(|b| b.position())
            .unique()
            .collect::<Vec<Point>>()
            .len()
    }
}

#[derive(PartialEq)]
enum Tile {
    Empty,
    SplitterVer,
    SplitterHor,
    MirrorFWD,
    MirrorBWD,
}

impl From<char> for Tile {
    fn from(value: char) -> Self {
        match value {
            '.' => Self::Empty,
            '|' => Self::SplitterVer,
            '-' => Self::SplitterHor,
            '/' => Self::MirrorFWD,
            '\\' => Self::MirrorBWD,
            _ => panic!("Unrecognized tile"),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::file_system::read_example;
    use crate::solutions::year2023::day16::Day16;
    use crate::solutions::Solution;

    #[test]
    fn part_one_example_test() {
        let input = read_example("16");

        assert_eq!("46", Day16.part_one(input.as_str()));
    }

    #[test]
    fn part_two_example_test() {
        let input = read_example("16");

        assert_eq!("51", Day16.part_two(input.as_str()));
    }
}
