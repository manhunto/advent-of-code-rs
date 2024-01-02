use std::fmt;
use std::fmt::Display;
use std::ops::{Div, Sub};
use Direction::{East, South, West, North};
use crate::direction::Direction;
use crate::grid::Grid;
use crate::point::Point;
use crate::shoelace_formula::{shoelace_formula};
use crate::solutions::Solution;

pub struct Day10;

impl Solution for Day10 {
    fn part_one(&self, input: &str) -> String {
        let grid: Grid<Tile> = Grid::from(input);
        let chain: Vec<Point> = self.walk(&grid);

        chain
            .len()
            .div(2)
            .to_string()
    }

    fn part_two(&self, input: &str) -> String {
        let grid: Grid<Tile> = Grid::from(input);
        let chain: Vec<Point> = self.walk(&grid);

        shoelace_formula(&chain)
            .sub(chain.len() as isize)
            .to_string()
    }
}

impl Day10 {
    fn walk(&self, grid: &Grid<Tile>) -> Vec<Point> {
        let start = grid.get_first_position(&Tile::Start).expect("No start point");

        let mut visited: Vec<Point> = vec![start];

        loop {
            let current = *visited.last().unwrap();
            let current_tile = grid.get_for_point(&current).expect("No tile?!");

            let adjacent: Vec<Point> = match current_tile {
                Tile::Start => current
                    .adjacent()
                    .into_iter()
                    .filter(|p| grid.is_in(p))
                    .filter(|adjacent| {
                        let tile = grid.get_for_point(&adjacent).unwrap();

                        adjacent.adjacent_in_directions(tile.directions()).contains(&current)
                    })
                    .collect(),
                _ => current
                    .adjacent_in_directions(current_tile.directions()),
            };

            let next_moves: Vec<Point> = adjacent
                .into_iter()
                .filter(|p| grid.is_in(p) && !visited.contains(&&p))
                .filter(|p| {
                    let tile = grid.get_for_point(&p).unwrap();

                    *tile != Tile::Ground
                })
                .collect();

            if visited.len() > 1 && next_moves.is_empty() {
                break;
            }

            let next_move = next_moves.first().expect("No next move").clone();

            visited.push(next_move);
        }

        visited
    }
}

#[derive(Debug, PartialEq, Copy, Clone)]
enum Tile {
    NS,
    EW,
    NE,
    NW,
    SW,
    SE,
    Ground,
    Start,
}

impl Tile {
    fn directions(&self) -> Vec<Direction> {
        match self {
            Tile::NS => { vec![North, South] }
            Tile::EW => { vec![East, West] }
            Tile::NE => { vec![North, East] }
            Tile::NW => { vec![North, West] }
            Tile::SW => { vec![South, West] }
            Tile::SE => { vec![South, East] }
            Tile::Ground => { vec![] }
            Tile::Start => { vec![South, East, West, North] }
        }
    }
}

impl From<char> for Tile {
    fn from(value: char) -> Self {
        match value {
            '|' => Self::NS,
            '-' => Self::EW,
            'L' => Self::NE,
            'J' => Self::NW,
            '7' => Self::SW,
            'F' => Self::SE,
            'S' => Self::Start,
            '.' => Self::Ground,
            _ => panic!("{}", format!("Unknown tile: {}", value))
        }
    }
}

impl Display for Tile {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let char: char = match self {
            Self::NS => '|',
            Self::EW => '-',
            Self::NE => 'L',
            Self::NW => 'J',
            Self::SW => '7',
            Self::SE => 'F',
            Self::Start => 'S',
            Self::Ground => '.',
        };

        write!(f, "{}", char)
    }
}

#[cfg(test)]
mod tests {
    use crate::file_system::read_example;
    use crate::point::Point;
    use crate::solutions::day10::{Day10, Tile};
    use crate::solutions::Solution;

    #[test]
    fn part_one_example_test() {
        let input = read_example("10");

        assert_eq!("4", Day10.part_one(&input.as_str()));
    }

    #[test]
    fn part_one_example_2_test() {
        let input = read_example("10_2");

        assert_eq!("4", Day10.part_one(&input.as_str()));
    }

    #[test]
    fn part_one_example_3_test() {
        let input = read_example("10_3");

        assert_eq!("8", Day10.part_one(&input.as_str()));
    }

    #[test]
    fn part_two_example_4_test() {
        let input = read_example("10_4");

        assert_eq!("4", Day10.part_two(&input.as_str()));
    }

    #[test]
    fn part_two_example_5_test() {
        let input = read_example("10_5");

        assert_eq!("8", Day10.part_two(&input.as_str()));
    }

    #[test]
    fn part_two_example_6_test() {
        let input = read_example("10_6");

        assert_eq!("10", Day10.part_two(&input.as_str()));
    }

    #[test]
    fn tile_in_direction() {
        let point = Point::new(1, 1);

        assert_eq!(vec![Point::new(1, 0), Point::new(1, 2)], point.adjacent_in_directions(Tile::from('|').directions()));
        assert_eq!(vec![Point::new(2, 1), Point::new(0, 1)], point.adjacent_in_directions(Tile::from('-').directions()));
        assert_eq!(vec![Point::new(1, 0), Point::new(2, 1)], point.adjacent_in_directions(Tile::from('L').directions()));
        assert_eq!(vec![Point::new(1, 0), Point::new(0, 1)], point.adjacent_in_directions(Tile::from('J').directions()));
        assert_eq!(vec![Point::new(1, 2), Point::new(0, 1)], point.adjacent_in_directions(Tile::from('7').directions()));
        assert_eq!(vec![Point::new(1, 2), Point::new(2, 1)], point.adjacent_in_directions(Tile::from('F').directions()));
    }
}
