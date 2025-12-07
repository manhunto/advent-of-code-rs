use crate::solutions::Solution;
use crate::utils::grid::{Grid, PrintableOnGrid};
use crate::utils::line::Line;
use crate::utils::point::Point;
use std::collections::{HashSet, VecDeque};

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

                for other in down.split() {
                    if result_beams.iter().any(|beam| beam.is_on(&other)) {
                        continue;
                    }

                    current_beams.push_back(other);
                }

                continue;
            }

            if down.current().y > max_height + 1 {
                result_beams.push(current_beam);
                continue;
            }

            current_beams.push_front(down);
        }

        let mut new = grid.clone();
        new.print(&result_beams[..]);

        println!("{}", new);

        result_beams.len().to_string()
    }

    fn part_two(&self, _input: &str) -> String {
        String::from("0")
    }
}

#[derive(Copy, Clone)]
struct Beam {
    line: Line,
}

impl Beam {
    fn is_on(&self, other: &Self) -> bool {
        if other.line.start() != other.line.end() {
            panic!("We only support beam that just started");
        }

        self.line.is_on(&other.line.start())
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

impl PrintableOnGrid for Beam {
    type Cell = char;

    fn print_on_grid(&self, grid: &mut Grid<char>) {
        let mut moved = self.line.start();

        loop {
            grid.modify(moved, BEAM);
            if moved == self.line.end() {
                break;
            }
            moved = moved.south();
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::solutions::year2025::day07::Day07;
    use crate::solutions::Solution;

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
}
