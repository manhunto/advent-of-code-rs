use std::collections::BTreeMap;
use std::fmt::{Display, Formatter};
use crate::grid::Grid;
use crate::point::Point;
use crate::solutions::Solution;

pub struct Day13;

impl Solution for Day13 {
    fn part_one(&self, input: &str) -> String {
        let grids: Vec<Grid<Type>> = input.split("\n\n").map(|part| Grid::from(part)).collect();

        grids
            .iter()
            .map(|grid| Self::find_mirror(grid.rows()) * 100 + Self::find_mirror(grid.columns()))
            .sum::<usize>()
            .to_string()
    }

    fn part_two(&self, input: &str) -> String {
        String::from("0")
    }
}

impl Day13 {
    fn find_mirror(rows_or_cols: BTreeMap<i32, BTreeMap<&Point, &Type>>) -> usize {
        for i in 0..rows_or_cols.len() - 1 {
            let is_mirror = (0..i + 1).filter_map(|j| {
                let a = i - j;
                let b = i + j + 1;

                if b >= rows_or_cols.len() {
                    return None;
                }

                let left: Vec<Type> = rows_or_cols.get(&(a as i32)).unwrap().into_iter().map(|(_, &&ref c)| c.clone()).collect();
                let right: Vec<Type> = rows_or_cols.get(&(b as i32)).unwrap().into_iter().map(|(_,&&ref c)| c.clone()).collect();

                Some(right == left)
            }).all(|t| t);

            if is_mirror {
                return i + 1;
            }
        }

        0
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
}
