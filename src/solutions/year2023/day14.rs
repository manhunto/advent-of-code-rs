use crate::solutions::Solution;
use crate::utils::direction::Direction;
use crate::utils::direction::Direction::{East, North, South, West};
use crate::utils::grid::Grid;
use crate::utils::point::Point;
use crate::utils::range::Range;
use crate::utils::surface_range::SurfaceRange;
use itertools::Itertools;
use std::collections::hash_map::DefaultHasher;
use std::collections::HashMap;
use std::hash::{Hash, Hasher};

pub struct Day14;

impl Solution for Day14 {
    fn part_one(&self, input: &str) -> String {
        let grid: Grid<char> = Grid::from(input);

        let rounded_rocks = Rocks::from(grid.get_all_positions(&'O'));
        let cube_rocks = Rocks::from(grid.get_all_positions(&'#'));

        let surface_range = grid.surface_range();

        let tilted = Self::tilt_north(surface_range, rounded_rocks, &cube_rocks);

        Self::total_load_on_north_support_beam(surface_range.rows(), tilted).to_string()
    }

    fn part_two(&self, input: &str) -> String {
        const NUMBER_OF_CYCLES: usize = 1_000_000_000;

        let grid: Grid<char> = Grid::from(input);
        let mut rounded_rocks = Rocks::from(grid.get_all_positions(&'O'));
        let cube_rocks = Rocks::from(grid.get_all_positions(&'#'));
        let surface_range = grid.surface_range();

        let mut history: Vec<u64> = Vec::new();
        let mut cycle_found = false;

        let mut current_cycle: usize = 1;

        while current_cycle < NUMBER_OF_CYCLES {
            rounded_rocks = Self::cycle(surface_range, rounded_rocks, &cube_rocks);

            if !cycle_found {
                let hash = rounded_rocks.hash();
                if let Some(position) = history.iter().position(|h| h == &hash) {
                    let diff = current_cycle - position - 1;
                    let cycles_left = NUMBER_OF_CYCLES - current_cycle;
                    let factor = cycles_left / diff;

                    current_cycle += factor * diff;
                    cycle_found = true;
                    continue;
                } else {
                    history.push(hash);
                }
            }

            current_cycle += 1;
        }

        Self::total_load_on_north_support_beam(surface_range.rows(), rounded_rocks).to_string()
    }
}

impl Day14 {
    fn total_load_on_north_support_beam(rows_range: Range, tilted: Rocks) -> usize {
        rows_range
            .iter()
            .map(|y| {
                let count = tilted.in_row(y).len();
                let row_number = rows_range.end() - y + 1;

                count * row_number as usize
            })
            .sum::<usize>()
    }

    fn cycle(range: SurfaceRange, rounded_rocks: Rocks, cube_rocks: &Rocks) -> Rocks {
        let north = Self::tilt_north(range, rounded_rocks, cube_rocks);
        let west = Self::tilt_west(range, north, cube_rocks);
        let south = Self::tilt_south(range, west, cube_rocks);

        Self::tilt_east(range, south, cube_rocks)
    }

    fn tilt_north(range: SurfaceRange, rounded_rocks: Rocks, cube_rocks: &Rocks) -> Rocks {
        let mut tilted = Rocks::new();

        for i in range.x().iter() {
            let rounded_rocks_in_column: Vec<Point> = rounded_rocks.in_column(i);
            let solid_rocks_in_line: Vec<Point> = cube_rocks.in_column(i);

            let tilted_in_line: Vec<Point> =
                Self::tilt_in_direction(range, rounded_rocks_in_column, solid_rocks_in_line, North);

            tilted.append(tilted_in_line);
        }

        tilted
    }

    fn tilt_south(range: SurfaceRange, rounded_rocks: Rocks, cube_rocks: &Rocks) -> Rocks {
        let mut tilted = Rocks::new();

        for i in range.x().rev_iter() {
            let rounded_rocks_in_column: Vec<Point> =
                rounded_rocks.in_column(i).into_iter().rev().collect();
            let solid_rocks_in_line: Vec<Point> =
                cube_rocks.in_column(i).into_iter().rev().collect();

            let tilted_in_line: Vec<Point> =
                Self::tilt_in_direction(range, rounded_rocks_in_column, solid_rocks_in_line, South);

            tilted.append(tilted_in_line);
        }

        tilted
    }

    fn tilt_west(range: SurfaceRange, rounded_rocks: Rocks, cube_rocks: &Rocks) -> Rocks {
        let mut tilted = Rocks::new();

        for i in range.y().iter() {
            let rounded_rocks_in_row: Vec<Point> = rounded_rocks.in_row(i);
            let solid_rocks_in_line: Vec<Point> = cube_rocks.in_row(i);

            let tilted_in_line: Vec<Point> =
                Self::tilt_in_direction(range, rounded_rocks_in_row, solid_rocks_in_line, West);

            tilted.append(tilted_in_line);
        }

        tilted
    }

    fn tilt_east(range: SurfaceRange, rounded_rocks: Rocks, cube_rocks: &Rocks) -> Rocks {
        let mut tilted = Rocks::new();

        for i in range.y().rev_iter() {
            let rounded_rocks_in_row: Vec<Point> =
                rounded_rocks.in_row(i).into_iter().rev().collect();
            let solid_rocks_in_line: Vec<Point> = cube_rocks.in_row(i).into_iter().rev().collect();

            let tilted_in_line: Vec<Point> =
                Self::tilt_in_direction(range, rounded_rocks_in_row, solid_rocks_in_line, East);

            tilted.append(tilted_in_line);
        }

        tilted
    }

    fn tilt_in_direction(
        range: SurfaceRange,
        rounded_rocks_in_column: Vec<Point>,
        solid_rocks_in_line: Vec<Point>,
        dir: Direction,
    ) -> Vec<Point> {
        let mut tilted_in_line: Vec<Point> = Vec::with_capacity(rounded_rocks_in_column.len());

        for rock in &rounded_rocks_in_column {
            let mut before = *rock;

            loop {
                let moved = before.move_in(dir);

                if solid_rocks_in_line.contains(&moved)
                    || tilted_in_line.contains(&moved)
                    || !range.contains(moved)
                {
                    tilted_in_line.push(before);
                    break;
                }

                before = moved;
            }
        }

        tilted_in_line
    }
}

struct Rocks {
    by_rows: HashMap<isize, Vec<Point>>,
    by_cols: HashMap<isize, Vec<Point>>,
    all: Vec<Point>,
}

impl Rocks {
    fn new() -> Self {
        Self {
            by_rows: HashMap::new(),
            by_cols: HashMap::new(),
            all: vec![],
        }
    }

    fn append(&mut self, rocks: Vec<Point>) {
        for rock in rocks {
            self.by_rows.entry(rock.y).or_default().push(rock);
            self.by_cols.entry(rock.x).or_default().push(rock);
            self.all.push(rock);
        }
    }

    fn in_column(&self, x: isize) -> Vec<Point> {
        self.by_cols
            .get(&x)
            .unwrap_or(&Vec::new())
            .clone()
            .into_iter()
            .sorted_by(|a, b| Ord::cmp(&a.y, &b.y))
            .collect()
    }

    fn in_row(&self, y: isize) -> Vec<Point> {
        self.by_rows
            .get(&y)
            .unwrap_or(&Vec::new())
            .clone()
            .into_iter()
            .sorted_by(|a, b| Ord::cmp(&a.x, &b.x))
            .collect()
    }

    fn hash(&self) -> u64 {
        let mut hasher = DefaultHasher::new();
        self.all.hash(&mut hasher);

        hasher.finish()
    }
}

impl From<Vec<Point>> for Rocks {
    fn from(value: Vec<Point>) -> Self {
        let mut rocks = Self::new();
        rocks.append(value);

        rocks
    }
}

#[cfg(test)]
mod tests {
    use crate::solutions::year2023::day14::{Day14, Rocks};
    use crate::solutions::year2023::read_2023_example;
    use crate::solutions::Solution;
    use crate::utils::grid::Grid;

    #[test]
    fn part_one_example_test() {
        let input = read_2023_example("14");

        assert_eq!("136", Day14.part_one(input.as_str()));
    }

    #[test]
    fn part_two_example_test() {
        let input = read_2023_example("14");

        assert_eq!("64", Day14.part_two(input.as_str()));
    }

    #[test]
    fn cycle_test() {
        let input = read_2023_example("14");

        let grid: Grid<char> = Grid::from(input.as_str());

        let after_first_cycle = cycle(grid);
        let expected = ".....#....
....#...O#
...OO##...
.OO#......
.....OOO#.
.O#...O#.#
....O#....
......OOOO
#...O###..
#..OO#....
";

        assert_eq!(expected, after_first_cycle.to_string());

        let after_second_cycle = cycle(after_first_cycle);
        let expected = ".....#....
....#...O#
.....##...
..O#......
.....OOO#.
.O#...O#.#
....O#...O
.......OOO
#..OO###..
#.OOO#...O
";

        assert_eq!(expected, after_second_cycle.to_string());

        let after_third_cycle = cycle(after_second_cycle);
        let expected = ".....#....
....#...O#
.....##...
..O#......
.....OOO#.
.O#...O#.#
....O#...O
.......OOO
#...O###.O
#.OOO#...O
";

        assert_eq!(expected, after_third_cycle.to_string());
    }

    fn cycle(grid: Grid<char>) -> Grid<char> {
        let rounded_rocks = Rocks::from(grid.get_all_positions(&'O'));
        let cube_rocks = Rocks::from(grid.get_all_positions(&'#'));

        let after_first_cycle = Day14::cycle(grid.surface_range(), rounded_rocks, &cube_rocks);

        let mut grid: Grid<char> = Grid::filled(grid.surface_range(), '.');
        grid.modify_many(after_first_cycle.all, 'O');
        grid.modify_many(cube_rocks.all, '#');

        grid
    }
}
