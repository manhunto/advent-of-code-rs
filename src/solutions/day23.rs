use crate::grid::Grid;
use crate::point::Point;
use crate::solutions::Solution;

pub struct Day23;

impl Solution for Day23 {
    fn part_one(&self, input: &str) -> String {
        let mut grid: Grid<char> = Grid::from(input);
        let start = Point::new(1, 0);
        let surface = grid.surface_range();
        let end = surface.bottom_right_corner().west();

        grid.modify(start, 'S');
        grid.modify(end, 'F');

        println!("{}", grid);

        String::from('0')
    }

    fn part_two(&self, _input: &str) -> String {
        String::from('0')
    }
}

#[cfg(test)]
mod tests {
    use crate::file_system::read_example;
    use crate::solutions::day23::Day23;
    use crate::solutions::Solution;

    #[test]
    fn part_one_example_test() {
        let input = read_example("23");

        assert_eq!("94", Day23.part_one(input.as_str()));
    }
}
