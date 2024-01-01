use crate::grid::Grid;
use crate::solutions::Solution;

pub struct Day17;

impl Solution for Day17 {
    fn part_one(&self, input: &str) -> String {
        let grid: Grid<u32> = Grid::from_custom(input, |c| c.to_digit(10).unwrap());

        println!("{}", grid);

        String::from('0')
    }

    fn part_two(&self, input: &str) -> String {
        String::from('0')
    }
}

#[cfg(test)]
mod tests {
    use crate::file_system::read_example;
    use crate::solutions::day17::Day17;
    use crate::solutions::Solution;

    #[test]
    fn part_one_example_test() {
        let input = read_example("17");

        assert_eq!("102", Day17.part_one(&input.as_str()));
    }
}
