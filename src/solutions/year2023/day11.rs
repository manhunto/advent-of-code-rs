use crate::solutions::Solution;
use crate::utils::grid::Grid;
use crate::utils::pair_generator::pairs;
use crate::utils::point::Point;
use std::cmp::{max, min};
use std::collections::BTreeMap;
use std::ops::Mul;

pub struct Day11;

impl Solution for Day11 {
    fn part_one(&self, input: &str) -> String {
        let grid: Grid<char> = self.parse_input(input);
        let galaxies = grid.get_all_positions(&'#');

        let pairs: Vec<(Point, Point)> = pairs(galaxies);

        pairs
            .iter()
            .map(|(a, b)| a.manhattan_distance(b))
            .sum::<isize>()
            .to_string()
    }

    fn part_two(&self, input: &str) -> String {
        self.solve_with_expanded_galaxy(input, 1_000_000)
    }
}

impl Day11 {
    fn parse_input(&self, input: &str) -> Grid<char> {
        let mut grid: Grid<char> = Grid::from(input);

        let rows = grid.rows();
        let rows_without_galaxy: Vec<isize> = self.get_empty(&rows);

        let columns = grid.columns();
        let columns_without_galaxy: Vec<isize> = self.get_empty(&columns);

        grid.insert_rows(rows_without_galaxy, '.');
        grid.insert_columns(columns_without_galaxy, '.');

        grid
    }

    fn get_empty<'a>(&'a self, data: &'a BTreeMap<isize, BTreeMap<&Point, &char>>) -> Vec<isize> {
        data.iter()
            .filter(|(_, element)| element.iter().all(|(_, &c)| c == &'.'))
            .map(|(i, _)| *i)
            .collect()
    }

    fn solve_with_expanded_galaxy(&self, input: &str, expand_by: isize) -> String {
        let grid: Grid<char> = Grid::from(input);

        let rows = grid.rows();
        let rows_without_galaxy: Vec<isize> = self.get_empty(&rows);

        let columns = grid.columns();
        let columns_without_galaxy: Vec<isize> = self.get_empty(&columns);

        let galaxies = grid.get_all_positions(&'#');

        let pairs: Vec<(Point, Point)> = pairs(galaxies);

        pairs
            .iter()
            .map(|(a, b)| {
                let from_x = min(a.x, b.x);
                let to_x = max(a.x, b.x);

                let between_x = columns_without_galaxy
                    .iter()
                    .filter(|x| (from_x..to_x).contains(x))
                    .collect::<Vec<&isize>>()
                    .len() as isize;

                let from_y = min(a.y, b.y);
                let to_y = max(a.y, b.y);

                let between_y = rows_without_galaxy
                    .iter()
                    .filter(|y| (from_y..to_y).contains(y))
                    .collect::<Vec<&isize>>()
                    .len() as isize;

                (a.manhattan_distance(b)
                    + between_x.mul(expand_by - 1)
                    + between_y.mul(expand_by - 1)) as i64
            })
            .sum::<i64>()
            .to_string()
    }
}

#[cfg(test)]
mod tests {
    use crate::solutions::year2023::day11::Day11;
    use crate::solutions::year2023::read_2023_example;
    use crate::solutions::Solution;

    #[test]
    fn parse_input() {
        let input = read_2023_example("11");

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

        assert_eq!(expected, Day11.parse_input(input.as_str()).to_string());
    }

    #[test]
    fn part_one_example_test() {
        let input = read_2023_example("11");

        assert_eq!("374", Day11.part_one(input.as_str()));
    }

    #[test]
    fn solve_with_expanded_galaxy_10_times() {
        let input = read_2023_example("11");

        assert_eq!("1030", Day11.solve_with_expanded_galaxy(input.as_str(), 10));
    }

    #[test]
    fn solve_with_expanded_galaxy_100_times() {
        let input = read_2023_example("11");

        assert_eq!(
            "8410",
            Day11.solve_with_expanded_galaxy(input.as_str(), 100)
        );
    }
}
