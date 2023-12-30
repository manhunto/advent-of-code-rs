use itertools::Itertools;
use crate::direction::Direction::North;
use crate::grid::Grid;
use crate::point::Point;
use crate::range::Range;
use crate::solutions::Solution;

pub struct Day14;

impl Solution for Day14 {
    fn part_one(&self, input: &str) -> String {
        let grid: Grid<char> = Grid::from(input);

        let rounded_rocks = grid.get_all_positions(&'O');
        let cube_rocks = grid.get_all_positions(&'#');
        let columns_range = grid.columns_range();
        let rows_range = grid.rows_range();

        let mut tilted: Vec<Point> = Vec::with_capacity(rounded_rocks.len());

        for i in columns_range.iter() {
            let rounded_rocks_in_column: Vec<Point> = Self::points_in_column(rounded_rocks.clone(), i as i32);
            let solid_rocks_in_column: Vec<Point> = Self::points_in_column(cube_rocks.clone(), i as i32);

            let mut tilted_in_column: Vec<Point> = Vec::with_capacity(rounded_rocks_in_column.len());

            for rock in &rounded_rocks_in_column {
                let mut before = rock.clone();

                loop {
                    let moved = before.move_in(North);

                    if solid_rocks_in_column.contains(&moved)
                        || tilted_in_column.contains(&moved)
                        || !columns_range.is_in_range(moved.y as i64)

                    {
                        tilted_in_column.push(before.clone());
                        break;
                    }

                    before = moved;
                }
            }

            tilted.append(&mut tilted_in_column);
        }

        Self::total_load_on_north_support_beam(rows_range, tilted).to_string()
    }

    fn part_two(&self, input: &str) -> String {
        String::from("0")
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
}

#[cfg(test)]
mod tests {
    use crate::file_system::read_example;
    use crate::solutions::day14::Day14;
    use crate::solutions::Solution;

    #[test]
    fn part_one_example_test() {
        let input = read_example("14");

        assert_eq!("136", Day14.part_one(&input.as_str()));
    }
}
