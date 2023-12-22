use std::collections::HashMap;
use itertools::Itertools;
use crate::grid::Grid;
use crate::pair_generator::pairs;
use crate::point::Point;
use crate::solutions::Solution;

pub struct Day11;

impl Solution for Day11 {
    fn part_one(&self, input: &str) -> String {
        let grid: Grid<char> = self.parse_input(input);
        let galaxies = grid.get_all_positions(&'#');

        let pairs: Vec<(Point, Point)> = pairs(galaxies);
        println!("{:?}", pairs.len());

        String::from('0')
    }

    fn part_two(&self, input: &str) -> String {
        String::from('0')
    }
}

impl Day11 {
    fn parse_input(&self, input: &str) -> Grid<char> {
        let mut grid: Grid<char> = Grid::from(input);

        let rows = grid.rows();
        let rows_without_galaxy: Vec<i32> = self.get_empty(&rows);

        let columns = grid.columns();
        let columns_without_galaxy: Vec<i32> = self.get_empty(&columns);

        for row in &rows_without_galaxy {
            grid.insert_row(row.clone(), '.')
        }

        for column in &columns_without_galaxy {
            grid.insert_column(column.clone(), '.')
        }

        grid
    }

    fn get_empty<'a>(&'a self, data: &'a HashMap<i32, HashMap<&Point, &char>>) -> Vec<i32> {
        data
            .iter()
            .filter(|(_, &ref element)| {
                element
                    .iter()
                    .all(|(_, &c)| c == &'.')
            })
            .map(|(i, _)| i.clone())
            .sorted()
            .rev()
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use crate::file_system::read_example;
    use crate::solutions::day11::Day11;
    use crate::solutions::Solution;

    #[test]
    fn parse_input() {
        let input = read_example("11");

        let expected = "....#........
.........#...
#............
.............
.............
........#....
.#...........
............#
.............
.............
.........#...
#....#.......
";

        assert_eq!(expected, Day11.parse_input(&input.as_str()).to_string());
    }

    #[test]
    fn part_one_example_test() {
        let input = read_example("11");

        assert_eq!("374", Day11.part_one(&input.as_str()));
    }
}
