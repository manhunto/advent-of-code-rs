use std::fmt::{Display, Formatter};
use crate::grid::Grid;
use crate::solutions::Solution;

pub struct Day13;

impl Solution for Day13 {
    fn part_one(&self, input: &str) -> String {

        let grids: Vec<Grid<Type>> = input.split("\n\n").map(|part| Grid::from(part)).collect();

        for part in grids {
            println!();
            println!();
            println!("{}", part);
        }

        String::from("0")
    }

    fn part_two(&self, input: &str) -> String {
        String::from("0")
    }
}

#[derive(PartialEq, Eq, Ord, PartialOrd)]
enum Type {
    Ash,
    Rock
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
    use crate::solutions::day13::Day13;
    use crate::solutions::Solution;

    #[test]
    fn part_one_example_test() {
        let input = read_example("13");

        assert_eq!("405", Day13.part_one(&input.as_str()));
    }
}
