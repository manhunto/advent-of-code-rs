use crate::grid::Grid;
use crate::solutions::Solution;

pub struct Day11;

impl Solution for Day11 {
    fn part_one(&self, input: &str) -> String {
        let grid: Grid<char> =  Grid::from(input);

        println!("{}", grid.to_string());

        String::from('0')
    }

    fn part_two(&self, input: &str) -> String {
        String::from('0')
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
