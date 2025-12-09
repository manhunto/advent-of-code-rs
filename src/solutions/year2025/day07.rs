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
        let (splits, _) = self.run(input);

        splits.to_string()
    }

    fn part_two(&self, _input: &str) -> String {
        String::from("0")
    }
}

impl Day07 {
    fn parse(&self, input: &str) -> Grid<char> {
        let without_redundant_lines = input.lines().step_by(2).collect::<Vec<_>>().join("\n");

        Grid::from(without_redundant_lines.as_str())
    }

    fn run(&self, input: &str) -> (u16, u16) {
        let grid = self.parse(input);
        let rows_range = grid.rows_range();
        let start = grid.get_first_position(&START).unwrap();

        let splitters: HashSet<Point> = grid.get_all_positions(&SPLITTER).into_iter().collect();

        let mut finished_beams: Vec<Beam> = Vec::new();
        let mut current_beams: VecDeque<Beam> = VecDeque::from(vec![start.into()]);
        let mut splits = 0u16;

        while let Some(current_beam) = current_beams.pop_front() {
            if finished_beams
                .iter()
                .chain(current_beams.iter())
                .any(|beam| beam.collides(&current_beam))
            {
                continue;
            }

            let down = current_beam.down();

            if splitters.contains(&down.current()) {
                finished_beams.push(current_beam);
                splits += 1;

                for split in down.split() {
                    current_beams.push_back(split);
                }

                continue;
            }

            if !rows_range.contains(down.current().y) {
                finished_beams.push(current_beam);
                continue;
            }

            current_beams.push_front(down);
        }

        (splits, 0)
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
        let other = other.current();

        let start = self.line.start();
        let end = self.line.end();

        start.x == other.x && (start.y..=end.y).contains(&other.y)
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

    fn print(&self, grid: &mut Grid<char>) {
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

    const EXAMPLE_FROM_REDDIT: &str = r#"..S..
.....
..^..
.....
...^.
.....
.^...
....."#;

    #[test]
    fn part_one_example_from_reddit() {
        assert_eq!("3", Day07.part_one(EXAMPLE_FROM_REDDIT));
    }

    const EXAMPLE_FROM_REDDIT2: &str = r#"..S..
.....
..^..
.....
...^.
.....
.^...
.....
..^..
....."#;

    #[test]
    fn part_one_example_from_reddit2() {
        assert_eq!("4", Day07.part_one(EXAMPLE_FROM_REDDIT2));
    }

    const MY_EXAMPLE: &str = r#"..S..
.....
..^..
.....
.^.^.
.....
.^.^.
....."#;

    #[test]
    fn part_one_my_example() {
        assert_eq!("3", Day07.part_one(MY_EXAMPLE));
    }

    const MY_EXAMPLE2: &str = r#"..S..
.....
..^..
.....
.^.^.
.....
..^..
.....
.^.^.
....."#;

    #[test]
    fn part_one_my_example2() {
        assert_eq!("6", Day07.part_one(MY_EXAMPLE2));
    }
}
