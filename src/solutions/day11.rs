use std::collections::HashMap;
use crate::grid::Grid;
use crate::point::Point;
use crate::solutions::Solution;

pub struct Day11;

impl Solution for Day11 {
    fn part_one(&self, input: &str) -> String {
        let grid = self.parse_input(&input);

        println!("{}", grid.to_string());

        String::from('0')
    }

    fn part_two(&self, input: &str) -> String {
        String::from('0')
    }
}

impl Day11 {
    fn parse_input(&self, input: &str) -> Grid<char> {
        let cells: HashMap<Point, char> = input
            .lines()
            .enumerate()
            .map(|(y, line)| -> Vec<(Point, char)> {
                line
                    .chars()
                    .enumerate()
                    .map(|(x, c)| (Point::new(x as i32, y as i32), c))
                    .collect()
            })
            .flatten()
            .collect();

        Grid::new(cells)
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
