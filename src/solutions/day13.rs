use std::collections::BTreeMap;
use std::fmt::{Display, Formatter};
use crate::grid::Grid;
use crate::point::Point;
use crate::solutions::Solution;

pub struct Day13;

impl Solution for Day13 {
    fn part_one(&self, input: &str) -> String {
        let grids: Vec<Grid<Type>> = Self::parse_input(input);

        grids
            .iter()
            .map(|grid| Self::find_mirror(grid.rows()) * 100 + Self::find_mirror(grid.columns()))
            .sum::<usize>()
            .to_string()
    }

    fn part_two(&self, input: &str) -> String {
        let grids: Vec<Grid<Type>> = Self::parse_input(input);

        grids
            .iter()
            .map(|grid| {
                let mut rows: Vec<usize> = Vec::new();
                let mut cols: Vec<usize> = Vec::new();

                for y in grid.rows_range().iter() {
                    for x in grid.columns_range().iter() {
                        let new_grid = Self::toggle_type(&grid, x, y);

                        let row = Self::find_mirror(new_grid.rows());
                        if row > 0 {
                            rows.push(row);
                        }

                        let column = Self::find_mirror(new_grid.columns());
                        if column > 0 {
                            cols.push(column);
                        }
                    }
                }

                let min_row = *rows.iter().min().unwrap_or(&usize::MAX);
                let min_cols = *cols.iter().min().unwrap_or(&usize::MAX);

                if min_row < min_cols {
                    return min_row * 100;
                }

                min_cols
            })
            .sum::<usize>()
            .to_string()
    }
}

impl Day13 {
    fn parse_input(input: &str) -> Vec<Grid<Type>>{
        input.split("\n\n").map(|part| Grid::from(part)).collect()
    }

    fn find_mirror(rows_or_cols: BTreeMap<i32, BTreeMap<&Point, &Type>>) -> usize {
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
                return i + 1;
            }
        }

        0
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

        assert_eq!(0, Day13::find_mirror(first_grid.rows()));
        assert_eq!(5, Day13::find_mirror(first_grid.columns()));

        let second_grid = grids.get(1).unwrap();
        assert_eq!(4, Day13::find_mirror(second_grid.rows()));
        assert_eq!(0, Day13::find_mirror(second_grid.columns()));
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
