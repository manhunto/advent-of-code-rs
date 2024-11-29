use crate::grid::Grid;
use crate::point::Point;
use crate::solutions::Solution;
use std::collections::HashSet;

pub struct Day21;

impl Solution for Day21 {
    fn part_one(&self, input: &str) -> String {
        Self::steps(input, 64)
    }

    fn part_two(&self, _input: &str) -> String {
        String::from('0')
    }
}

impl Day21 {
    fn steps(input: &str, count: usize) -> String {
        let grid: Grid<char> = Grid::from(input);

        let start = grid.get_first_position(&'S').unwrap();
        let surface = grid.surface_range();
        let rocks = grid.get_all_positions(&'#');

        let mut reached: HashSet<Point> = HashSet::from([start]);

        for _ in 1..=count {
            let mut new_reached: HashSet<Point> = HashSet::new();
            for point in &reached {
                for adj in point.adjacent() {
                    if surface.contains(adj) && !rocks.contains(&adj) {
                        new_reached.insert(adj);
                    }
                }
            }

            reached = new_reached;
        }

        reached.len().to_string()
    }
}

#[cfg(test)]
mod tests {
    use crate::solutions::year2023::read_2023_example;
    use crate::solutions::year2023::day21::Day21;

    #[test]
    fn part_one_example_test() {
        let input = read_2023_example("21");

        assert_eq!("16", Day21::steps(input.as_str(), 6));
    }
}
