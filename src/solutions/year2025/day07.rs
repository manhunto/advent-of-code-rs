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

type Splits = u16;
type Timelines = u64;

impl Solution for Day07 {
    fn part_one(&self, input: &str) -> String {
        let (splits, _) = self.run(input);

        splits.to_string()
    }

    fn part_two(&self, input: &str) -> String {
        let (_, timelines) = self.run(input);

        timelines.to_string()
    }
}

impl Day07 {
    fn parse(&self, input: &str) -> Grid<char> {
        let without_redundant_lines = input.lines().step_by(2).collect::<Vec<_>>().join("\n");

        Grid::from(without_redundant_lines.as_str())
    }

    fn run(&self, input: &str) -> (Splits, Timelines) {
        let grid = self.parse(input);
        let rows_range = grid.rows_range();
        let start = grid.get_first_position(&START).unwrap();

        let splitters: HashSet<Point> = grid.get_all_positions(&SPLITTER).into_iter().collect();

        let mut split_beams: Vec<Beam> = Vec::new();
        let mut finished_beams: Vec<Beam> = Vec::new();
        let mut current_beams: VecDeque<Beam> = VecDeque::from(vec![Beam::new(start)]);

        'while_loop: while let Some(current_beam) = current_beams.pop_front() {
            for beam in split_beams
                .iter_mut()
                .chain(current_beams.iter_mut())
                .chain(finished_beams.iter_mut())
            {
                if current_beam.collides(beam) {
                    *beam = beam.merge(current_beam);
                    continue 'while_loop;
                }
            }

            let down = current_beam.down();

            if splitters.contains(&down.current()) {
                split_beams.push(current_beam);

                for split in down.split() {
                    current_beams.push_back(split);
                }

                continue;
            }

            if !rows_range.contains(down.current().y) {
                finished_beams.push(current_beam);
                continue;
            }

            current_beams.push_back(down);
        }

        (
            split_beams.len() as Splits,
            finished_beams
                .iter()
                .fold(0, |acc, beam| acc + beam.timelines),
        )
    }
}

#[expect(dead_code)]
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
    timelines: Timelines,
}

impl Beam {
    fn new(point: Point) -> Self {
        Self {
            line: Line::new(point, point),
            timelines: 1,
        }
    }

    fn collides(&self, other: &Self) -> bool {
        let other = other.current();

        let start = self.line.start();
        let end = self.line.end();

        start.x == other.x && (start.y..=end.y).contains(&other.y)
    }

    fn down(&self) -> Self {
        Self {
            line: Line::new(self.line.start(), self.line.end().south()),
            timelines: self.timelines,
        }
    }

    fn current(&self) -> Point {
        self.line.end()
    }

    fn split(&self) -> Vec<Beam> {
        let west = self.current().west();
        let east = self.current().east();

        vec![
            Self {
                line: Line::new(west, west),
                timelines: self.timelines,
            },
            Self {
                line: Line::new(east, east),
                timelines: self.timelines,
            },
        ]
    }

    fn merge(&self, other: Self) -> Self {
        Self {
            line: self.line,
            timelines: self.timelines + other.timelines,
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
    fn part_two_example_test() {
        assert_eq!("40", Day07.part_two(EXAMPLE));
    }

    #[test]
    fn beam_collides() {
        let beam: Beam = Beam::new(Point::new(3, 0)).down().down().down();

        assert!(!beam.collides(&Beam::new(Point::new(3, -1))));
        assert!(beam.collides(&Beam::new(Point::new(3, 0))));
        assert!(beam.collides(&Beam::new(Point::new(3, 1))));
        assert!(beam.collides(&Beam::new(Point::new(3, 2))));
        assert!(beam.collides(&Beam::new(Point::new(3, 3))));
        assert!(!beam.collides(&Beam::new(Point::new(3, 4))));

        assert!(!beam.collides(&Beam::new(Point::new(2, 0))));
        assert!(!beam.collides(&Beam::new(Point::new(2, 1))));
        assert!(!beam.collides(&Beam::new(Point::new(2, 2))));
        assert!(!beam.collides(&Beam::new(Point::new(2, 3))));
        assert!(!beam.collides(&Beam::new(Point::new(2, 4))));

        assert!(!beam.collides(&Beam::new(Point::new(4, 0))));
        assert!(!beam.collides(&Beam::new(Point::new(4, 1))));
        assert!(!beam.collides(&Beam::new(Point::new(4, 2))));
        assert!(!beam.collides(&Beam::new(Point::new(4, 3))));
        assert!(!beam.collides(&Beam::new(Point::new(4, 4))));
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

    #[test]
    fn part_two_example_from_reddit2() {
        assert_eq!("6", Day07.part_two(EXAMPLE_FROM_REDDIT2));
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

    #[test]
    fn part_two_my_example() {
        assert_eq!("4", Day07.part_two(MY_EXAMPLE));
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

    const MY_EXAMPLE3: &str = r#"...S...
.......
...^...
.......
..^.^..
.......
.^...^.
......."#;

    #[test]
    fn part_two_my_example3() {
        assert_eq!("6", Day07.part_two(MY_EXAMPLE3));
    }

    const MY_EXAMPLE3_EXTENDED: &str = r#"...S...
.......
...^...
.......
..^.^..
.......
.^...^.
.......
...^...
......."#;

    #[test]
    fn part_two_my_example3_extended() {
        assert_eq!("8", Day07.part_two(MY_EXAMPLE3_EXTENDED));
    }
}
