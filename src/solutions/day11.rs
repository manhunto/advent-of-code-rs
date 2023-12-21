use std::collections::HashMap;
use itertools::Itertools;
use crate::grid::Grid;
use crate::point::Point;
use crate::solutions::Solution;

pub struct Day11;

impl Solution for Day11 {
    fn part_one(&self, input: &str) -> String {
        let grid: Grid<char> =  Grid::from(input);

        let rows = grid.rows();
        let rows_without_galaxy: Vec<&i32> = self.get_empty(&rows);

        let columns = grid.columns();
        let columns_without_galaxy: Vec<&i32> = self.get_empty(&columns);

        println!("{}", grid.to_string());

        println!("{:?}", rows_without_galaxy);
        println!("{:?}", columns_without_galaxy);

        String::from('0')
    }

    fn part_two(&self, input: &str) -> String {
        String::from('0')
    }
}

impl Day11 {
    fn get_empty<'a>(&'a self, data: &'a HashMap<i32, HashMap<&Point, &char>>) -> Vec<&i32>{
        data
            .iter()
            .filter(|(_, &ref element)| {
                element
                    .iter()
                    .all(|(_, &c)| c == &'.')
            })
            .map(|(i, _)| i)
            .sorted()
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use crate::file_system::read_example;
    use crate::solutions::day11::Day11;
    use crate::solutions::Solution;

    #[test]
    fn part_one_example_test() {
        let input = read_example("11");

        assert_eq!("374", Day11.part_one(&input.as_str()));
    }
}
