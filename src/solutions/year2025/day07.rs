use crate::solutions::Solution;
use crate::utils::grid::{Grid, PrintableOnGrid};
use crate::utils::line::Line;
use crate::utils::point::Point;
use std::collections::{HashSet, VecDeque};
use std::fmt::{Debug, Display, Formatter};

const START: char = 'S';
const SPLITTER: char = '^';
const BEAM: char = '|';

pub struct Day07;

impl Solution for Day07 {
    fn part_one(&self, input: &str) -> String {
        let grid: Grid<char> = Grid::from(input);
        let max_height = grid.columns_range().end();
        let start = grid.get_first_position(&START).unwrap();

        let splitters: HashSet<Point> = grid.get_all_positions(&SPLITTER).into_iter().collect();

        let mut result_beams: Vec<Beam> = Vec::new();
        let mut current_beams: VecDeque<Beam> = VecDeque::from(vec![start.into()]);

        while let Some(current_beam) = current_beams.pop_front() {
            let down = current_beam.down();

            if splitters.contains(&down.current()) {
                result_beams.push(current_beam);

                for split in down.split() {
                    if result_beams.iter().any(|beam| beam.collides(&split)) {
                        continue;
                    }

                    if current_beams.iter().any(|beam| beam.collides(&split)) {
                        continue;
                    }

                    current_beams.push_back(split);
                }

                continue;
            }

            if down.current().y > max_height + 1 {
                continue;
            }

            current_beams.push_front(down);
        }

        result_beams.len().to_string()
    }

    fn part_two(&self, _input: &str) -> String {
        String::from("0")
    }
}

#[allow(dead_code)]
fn print(grid: &Grid<char>, beams: &[Beam]) {
    let mut new = grid.clone();
    new.print(beams);

    println!("{}", new);
    println!("{} beams", beams.len());
    println!();
}

#[derive(Copy, Clone, Debug)]
struct Beam {
    line: Line,
}

impl Beam {
    fn collides(&self, other: &Self) -> bool {
        let other_start = other.line.start();
        if other_start != other.line.end() {
            panic!("We only support beam that just started");
        }

        let start = self.line.start();
        let end = self.line.end();

        if start.x != other_start.x {
            return false;
        }

        start.y <= other_start.y && end.y >= other_start.y
    }

    fn down(&self) -> Self {
        Self {
            line: Line::new(self.line.start(), self.line.end().south()),
        }
    }

    fn current(&self) -> Point {
        self.line.end()
    }

    fn split(&self) -> Vec<Beam> {
        vec![self.current().west().into(), self.current().east().into()]
    }
}

impl From<Point> for Beam {
    fn from(value: Point) -> Self {
        Self {
            line: Line::new(value, value),
        }
    }
}

impl From<(Point, Point)> for Beam {
    fn from(value: (Point, Point)) -> Self {
        Self {
            line: Line::new(value.0, value.1),
        }
    }
}

impl Display for Beam {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}) - ({})", self.line.start(), self.line.end())
    }
}

impl PrintableOnGrid for Beam {
    type Cell = char;

    fn print_on_grid(&self, grid: &mut Grid<char>) {
        let mut current = self.line.start();

        loop {
            grid.modify(current, BEAM);
            if current == self.line.end() {
                break;
            }
            current = current.south();
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::solutions::year2025::day07::{Beam, Day07};
    use crate::solutions::Solution;
    use crate::utils::point::Point;

    const EXAMPLE: &str = r#".......S.......
...............
.......^.......
...............
......^.^......
...............
.....^.^.^.....
...............
....^.^...^....
...............
...^.^...^.^...
...............
..^...^.....^..
...............
.^.^.^.^.^...^.
..............."#;

    #[test]
    fn part_one_example_test() {
        assert_eq!("21", Day07.part_one(EXAMPLE));
    }

    #[test]
    fn beam_collides() {
        let beam: Beam = (Point::new(3, 0), Point::new(3, 3)).into();

        assert!(!beam.collides(&Beam::from(Point::new(3, -1))));
        assert!(beam.collides(&Beam::from(Point::new(3, 0))));
        assert!(beam.collides(&Beam::from(Point::new(3, 1))));
        assert!(beam.collides(&Beam::from(Point::new(3, 2))));
        assert!(beam.collides(&Beam::from(Point::new(3, 3))));
        assert!(!beam.collides(&Beam::from(Point::new(3, 4))));

        assert!(!beam.collides(&Beam::from(Point::new(2, 0))));
        assert!(!beam.collides(&Beam::from(Point::new(2, 1))));
        assert!(!beam.collides(&Beam::from(Point::new(2, 2))));
        assert!(!beam.collides(&Beam::from(Point::new(2, 3))));
        assert!(!beam.collides(&Beam::from(Point::new(2, 4))));

        assert!(!beam.collides(&Beam::from(Point::new(4, 0))));
        assert!(!beam.collides(&Beam::from(Point::new(4, 1))));
        assert!(!beam.collides(&Beam::from(Point::new(4, 2))));
        assert!(!beam.collides(&Beam::from(Point::new(4, 3))));
        assert!(!beam.collides(&Beam::from(Point::new(4, 4))));
    }
}
