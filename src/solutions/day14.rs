use itertools::{Itertools};
use crate::direction::Direction;
use crate::direction::Direction::{East, North, South, West};
use crate::grid::Grid;
use crate::point::Point;
use crate::range::Range;
use crate::solutions::Solution;
use crate::utils::surface_range::SurfaceRange;

pub struct Day14;

impl Solution for Day14 {
    fn part_one(&self, input: &str) -> String {
        let grid: Grid<char> = Grid::from(input);

        let rounded_rocks = grid.get_all_positions(&'O');
        let cube_rocks = grid.get_all_positions(&'#');

        let surface_range = grid.surface_range();

        let tilted: Vec<Point> = Self::tilt_north(surface_range, rounded_rocks, cube_rocks);

        Self::total_load_on_north_support_beam(surface_range.rows(), tilted).to_string()
    }

    fn part_two(&self, input: &str) -> String {
        let grid: Grid<char> = Grid::from(input);

        let mut rounded_rocks = grid.get_all_positions(&'O');
        let cube_rocks = grid.get_all_positions(&'#');

        let surface_range = grid.surface_range();

        for c in 0..1_000_000_000 {
            rounded_rocks = Self::cycle(surface_range, rounded_rocks, cube_rocks.clone())
        }

        Self::total_load_on_north_support_beam(surface_range.rows(), rounded_rocks).to_string()
    }
}

impl Day14 {
    fn points_in_column(points: Vec<Point>, x: i32) -> Vec<Point> {
        points
            .into_iter()
            .filter(|p| p.x == x)
            .sorted_by(|a, b| Ord::cmp(&a.y, &b.y))
            .collect()
    }

    fn points_in_row(points: Vec<Point>, y: i32) -> Vec<Point> {
        points
            .into_iter()
            .filter(|p| p.y == y)
            .sorted_by(|a, b| Ord::cmp(&a.x, &b.x))
            .collect()
    }

    fn total_load_on_north_support_beam(rows_range: Range, tilted: Vec<Point>) -> usize {
        rows_range
            .iter()
            .map(|y| {
                let count = tilted.iter().filter(|p|p.y == y as i32).collect::<Vec<&Point>>().len();
                let row_number = rows_range.end() - y + 1;

                count * row_number as usize
            })
            .sum::<usize>()
    }

    fn cycle(range: SurfaceRange, rounded_rocks: Vec<Point>, cube_rocks: Vec<Point>) -> Vec<Point> {
        let north = Self::tilt_north(range, rounded_rocks, cube_rocks.clone());
        let west = Self::tilt_west(range, north, cube_rocks.clone());
        let south = Self::tilt_south(range, west, cube_rocks.clone());
        let east = Self::tilt_east(range, south, cube_rocks.clone());
        
        east
    }

    fn tilt_north(range: SurfaceRange, rounded_rocks: Vec<Point>, cube_rocks: Vec<Point>) -> Vec<Point> {
        let mut tilted: Vec<Point> = Vec::with_capacity(rounded_rocks.len());

        for i in range.x().iter() {
            let rounded_rocks_in_column: Vec<Point> = Self::points_in_column(rounded_rocks.clone(), i as i32);
            let solid_rocks_in_line: Vec<Point> = Self::points_in_column(cube_rocks.clone(), i as i32);

            let mut tilted_in_line: Vec<Point> = Self::tilt_in_direction(range, rounded_rocks_in_column, solid_rocks_in_line, North);

            tilted.append(&mut tilted_in_line);
        }

        tilted
    }

    fn tilt_south(range: SurfaceRange, rounded_rocks: Vec<Point>, cube_rocks: Vec<Point>) -> Vec<Point> {
        let mut tilted: Vec<Point> = Vec::with_capacity(rounded_rocks.len());

        for i in range.x().iter().into_iter().collect::<Vec<i64>>().into_iter().rev() {
            let rounded_rocks_in_column: Vec<Point> = Self::points_in_column(rounded_rocks.clone(), i as i32).into_iter().rev().collect();
            let solid_rocks_in_line: Vec<Point> = Self::points_in_column(cube_rocks.clone(), i as i32).into_iter().rev().collect();

            let mut tilted_in_line: Vec<Point> = Self::tilt_in_direction(range, rounded_rocks_in_column, solid_rocks_in_line, South);

            tilted.append(&mut tilted_in_line);
        }

        tilted
    }

    fn tilt_west(range: SurfaceRange, rounded_rocks: Vec<Point>, cube_rocks: Vec<Point>) -> Vec<Point> {
        let mut tilted: Vec<Point> = Vec::with_capacity(rounded_rocks.len());

        for i in range.y().iter() {
            let rounded_rocks_in_column: Vec<Point> = Self::points_in_row(rounded_rocks.clone(), i as i32);
            let solid_rocks_in_line: Vec<Point> = Self::points_in_row(cube_rocks.clone(), i as i32);

            let mut tilted_in_line: Vec<Point> = Self::tilt_in_direction(range, rounded_rocks_in_column, solid_rocks_in_line, West);

            tilted.append(&mut tilted_in_line);
        }

        tilted
    }

    fn tilt_east(range: SurfaceRange, rounded_rocks: Vec<Point>, cube_rocks: Vec<Point>) -> Vec<Point> {
        let mut tilted: Vec<Point> = Vec::with_capacity(rounded_rocks.len());

        for i in range.y().iter().into_iter().collect::<Vec<i64>>().into_iter().rev() {
            let rounded_rocks_in_column: Vec<Point> = Self::points_in_row(rounded_rocks.clone(), i as i32).into_iter().rev().collect();
            let solid_rocks_in_line: Vec<Point> = Self::points_in_row(cube_rocks.clone(), i as i32).into_iter().rev().collect();

            let mut tilted_in_line: Vec<Point> = Self::tilt_in_direction(range, rounded_rocks_in_column, solid_rocks_in_line, East);

            tilted.append(&mut tilted_in_line);
        }

        tilted
    }

    fn tilt_in_direction(range: SurfaceRange, rounded_rocks_in_column: Vec<Point>, solid_rocks_in_line: Vec<Point>, dir: Direction) -> Vec<Point> {
        let mut tilted_in_line: Vec<Point> = Vec::with_capacity(rounded_rocks_in_column.len());

        for rock in &rounded_rocks_in_column {
            let mut before = rock.clone();

            loop {
                let moved = before.move_in(dir);

                if solid_rocks_in_line.contains(&moved)
                    || tilted_in_line.contains(&moved)
                    || !range.contains(moved)

                {
                    tilted_in_line.push(before.clone());
                    break;
                }

                before = moved;
            }
        }

        tilted_in_line
    }
}

#[cfg(test)]
mod tests {
    use crate::file_system::read_example;
    use crate::grid::Grid;
    use crate::solutions::day14::Day14;
    use crate::solutions::Solution;

    #[test]
    fn part_one_example_test() {
        let input = read_example("14");

        assert_eq!("136", Day14.part_one(&input.as_str()));
    }

    // #[test]
    // fn part_two_example_test() {
    //     let input = read_example("14");
    //
    //     assert_eq!("64", Day14.part_two(&input.as_str()));
    // }

    #[test]
    fn cycle_test() {
        let input = read_example("14");

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
        let rounded_rocks = grid.get_all_positions(&'O');
        let cube_rocks = grid.get_all_positions(&'#');

        let after_first_cycle = Day14::cycle(grid.surface_range(), rounded_rocks, cube_rocks.clone());

        let mut grid: Grid<char> = Grid::filled(grid.surface_range(), '.');
        grid.modify_many(after_first_cycle, 'O');
        grid.modify_many(cube_rocks.clone(), '#');

        grid
    }
}
