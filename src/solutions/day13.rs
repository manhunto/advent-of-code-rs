use std::collections::BTreeMap;
use std::fmt::{Display, Formatter};
use crate::grid::Grid;
use crate::point::Point;
use crate::range::Range;
use crate::solutions::Solution;

pub struct Day13;

impl Solution for Day13 {
    fn part_one(&self, input: &str) -> String {
        let grids: Vec<Grid<Type>> = Self::parse_input(input);

        grids
            .iter()
            .map(|grid| {
                Self::find_mirror(grid.rows()).unwrap_or(0) * 100
                    + Self::find_mirror(grid.columns()).unwrap_or(0)
            })
            .sum::<usize>()
            .to_string()
    }

    fn part_two(&self, input: &str) -> String {
        let grids: Vec<Grid<Type>> = Self::parse_input(input);

        grids
            .iter()
            .map(|grid| {
                let rows_range = grid.rows_range();
                for y in rows_range.iter() {
                    let columns_range = grid.columns_range();
                    for x in columns_range.iter() {
                        let new_grid = Self::toggle_type(&grid, x, y);

                        if let Some(row) = Self::find_mirror(new_grid.rows()) {
                            if Self::is_position_in_reflection(row, rows_range.end() as usize, y as usize) {
                                return row * 100;
                            }
                        }

                        if let Some(column) = Self::find_mirror(new_grid.columns()) {
                            if Self::is_position_in_reflection(column, columns_range.end() as usize, x as usize) {
                                return column;
                            }
                        }
                    }
                }

                Self::find_mirror(grid.rows()).unwrap_or(0) * 100
                    + Self::find_mirror(grid.columns()).unwrap_or(0)
            })
            .sum::<usize>()
            .to_string()
    }
}

impl Day13 {
    fn parse_input(input: &str) -> Vec<Grid<Type>> {
        input.split("\n\n").map(|part| Grid::from(part)).collect()
    }

    fn find_mirror(rows_or_cols: BTreeMap<i32, BTreeMap<&Point, &Type>>) -> Option<usize> {
        for i in 0..rows_or_cols.len() - 1 {
            let is_mirror = (0..i + 1).filter_map(|j| {
                let a = i - j;
                let b = i + j + 1;

                if b >= rows_or_cols.len() {
                    return None;
                }

                let left: Vec<Type> = Self::get_values(&rows_or_cols, a);
                let right: Vec<Type> = Self::get_values(&rows_or_cols, b);

                Some(right == left)
            }).all(|t| t);

            if is_mirror {
                return Some(i + 1);
            }
        }

        None
    }

    fn get_values(data: &BTreeMap<i32, BTreeMap<&Point, &Type>>, index: usize) -> Vec<Type> {
        data.get(&(index as i32)).unwrap().into_iter().map(|(_, &&ref c)| c.clone()).collect()
    }

    fn toggle_type(grid: &Grid<Type>, x: i64, y: i64) -> Grid<Type> {
        let new_type = match grid.get(x as i32, y as i32).unwrap() {
            Type::Ash => Type::Rock,
            Type::Rock => Type::Ash,
        };

        let mut new_grid = grid.clone();
        new_grid.modify(Point::new(x as i32, y as i32), new_type);

        new_grid
    }

    fn is_position_in_reflection(reflection_at: usize, max_position: usize, changed_position: usize) -> bool {
        let index = reflection_at - 1;
        let reflection_length = index.min(max_position - index - 1);
        let reflection_range = Range::new(
            (reflection_at - reflection_length) as i64,
            (reflection_at + reflection_length) as i64)
            .unwrap();

        reflection_range.is_in_range(changed_position as i64)
    }
}

#[derive(PartialEq, Eq, Ord, PartialOrd, Clone, Debug)]
enum Type {
    Ash,
    Rock,
}

impl From<char> for Type {
    fn from(value: char) -> Self {
        match value {
            '.' => Type::Ash,
            '#' => Type::Rock,
            _ => panic!("Unknown type")
        }
    }
}

impl Display for Type {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let c = match self {
            Type::Ash => '.',
            Type::Rock => '#'
        };
        write!(f, "{}", c)
    }
}

#[cfg(test)]
mod tests {
    use crate::file_system::read_example;
    use crate::grid::Grid;
    use crate::solutions::day13::{Day13, Type};
    use crate::solutions::Solution;

    #[test]
    fn part_one_example_test() {
        let input = read_example("13");

        assert_eq!("405", Day13.part_one(&input.as_str()));
    }

    #[test]
    fn part_two_example_test() {
        let input = read_example("13");

        assert_eq!("400", Day13.part_two(&input.as_str()));
    }

    #[test]
    fn find_mirror_test() {
        let input = read_example("13");

        let grids: Vec<Grid<Type>> = input.split("\n\n").map(|part| Grid::from(part)).collect();
        let first_grid = grids.get(0).unwrap();

        assert_eq!(None, Day13::find_mirror(first_grid.rows()));
        assert_eq!(Some(5), Day13::find_mirror(first_grid.columns()));

        let second_grid = grids.get(1).unwrap();
        assert_eq!(Some(4), Day13::find_mirror(second_grid.rows()));
        assert_eq!(None, Day13::find_mirror(second_grid.columns()));
    }

    #[test]
    fn toggle_type_test() {
        let input = "#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.";
        let grid: Grid<Type> = Grid::from(input);
        let new_grid = Day13::toggle_type(&grid, 0, 0);

        let expected = "..##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.
";

        assert_eq!(expected, new_grid.to_string());

        let new_grid = Day13::toggle_type(&grid, 2, 1);

        let expected = "#.##..##.
....##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.
";

        assert_eq!(expected, new_grid.to_string());
    }
}
