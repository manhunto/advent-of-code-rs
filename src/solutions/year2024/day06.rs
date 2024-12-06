use crate::solutions::Solution;
use crate::utils::direction::Direction::North;
use crate::utils::grid::Grid;
use crate::utils::point::Point;
use crate::utils::vector::Vector;
use std::collections::HashSet;

pub struct Day06;

impl Solution for Day06 {
    fn part_one(&self, input: &str) -> String {
        let grid: Grid<char> = Grid::from(input);
        let obstructions = grid.get_all_positions(&'#');
        let guard = grid.get_first_position(&'^').unwrap();
        let surface = grid.surface_range();

        let mut guard = Vector::new(guard, North);
        let mut visited_positions: HashSet<Point> = HashSet::new();

        while surface.contains(guard.position()) {
            visited_positions.insert(guard.position());

            let next_position = guard.step();
            if obstructions.contains(&next_position.position()) {
                guard = guard.rotate_cw().step()
            } else {
                guard = next_position;
            }
        }

        visited_positions.len().to_string()
    }

    fn part_two(&self, _input: &str) -> String {
        String::from('0')
    }
}

#[cfg(test)]
mod tests {
    use crate::solutions::year2024::day06::Day06;
    use crate::solutions::Solution;

    const EXAMPLE: &str = r#"....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#..."#;

    #[test]
    fn part_one_example_test() {
        assert_eq!("41", Day06.part_one(EXAMPLE));
    }
}
