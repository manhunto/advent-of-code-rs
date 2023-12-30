use itertools::{Itertools};
use Direction::{East, North, South, West};
use crate::direction::Direction;
use crate::grid::Grid;
use crate::point::Point;
use crate::solutions::Solution;
use crate::utils::vector::Vector;

pub struct Day16;

impl Solution for Day16 {
    fn part_one(&self, input: &str) -> String {
        let grid: Grid<char> = Grid::from(input);

        Self::energize(Vector::new(Point::new(0, 0), East), &grid).to_string()
    }

    fn part_two(&self, input: &str) -> String {
        let grid: Grid<char> = Grid::from(input);
        let surface_range = grid.surface_range();

        let starting_points: Vec<Vector> = surface_range.vectors_pointing_inwards();

        starting_points
            .into_iter()
            .map(|start| Self::energize(start, &grid))
            .max()
            .unwrap()
            .to_string()
    }
}

impl Day16 {
    fn energize(start: Vector, grid: &Grid<char>) -> usize {
        let surface_range = grid.surface_range();

        let mut beams: Vec<Vector> = vec![start];
        let mut history: Vec<Vector> = Vec::new();

        while !beams.is_empty() {
            beams = beams
                .into_iter()
                .filter_map(|beam| {
                    let position = beam.position();
                    if !surface_range.contains(position) || history.contains(&beam){
                        return None;
                    }

                    history.push(beam.clone());

                    let tile = grid.get_for_point(&position).unwrap();
                    let direction = beam.direction();

                    Some(match *tile {
                        '.' => vec![beam.do_move()],
                        '|' => match direction {
                            South | North => vec![beam.do_move()],
                            _ => vec![beam.rotate(South).do_move(), beam.rotate(North).do_move()]
                        },
                        '-' => match direction {
                            East | West => vec![beam.do_move()],
                            _ => vec![beam.rotate(East).do_move(), beam.rotate(West).do_move()]
                        },
                        '/' => vec![match direction {
                            North => beam.rotate(East).do_move(),
                            East => beam.rotate(North).do_move(),
                            West => beam.rotate(South).do_move(),
                            South => beam.rotate(West).do_move(),
                        }],
                        '\\' => vec![match direction {
                            North => beam.rotate(West).do_move(),
                            East => beam.rotate(South).do_move(),
                            West => beam.rotate(North).do_move(),
                            South => beam.rotate(East).do_move(),
                        }],
                        _ => panic!("{}", format!("Unrecognized {} {} {:?}", tile, position, direction))
                    })
                })
                .flatten()
                .collect();
        }

        history
            .clone()
            .iter()
            .map(|b| b.position())
            .unique()
            .collect::<Vec<Point>>()
            .len()
    }
}

#[cfg(test)]
mod tests {
    use crate::file_system::read_example;
    use crate::solutions::day16::Day16;
    use crate::solutions::Solution;

    #[test]
    fn part_one_example_test() {
        let input = read_example("16");

        assert_eq!("46", Day16.part_one(&input.as_str()));
    }

    #[test]
    fn part_two_example_test() {
        let input = read_example("16");

        assert_eq!("51", Day16.part_two(&input.as_str()));
    }
}
