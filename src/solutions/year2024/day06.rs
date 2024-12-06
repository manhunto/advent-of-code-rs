use crate::solutions::Solution;
use crate::utils::direction::Direction::North;
use crate::utils::grid::Grid;
use crate::utils::point::Point;
use crate::utils::vector::Vector;
use itertools::Itertools;

pub struct Day06;

impl Solution for Day06 {
    fn part_one(&self, input: &str) -> String {
        let grid: Grid<char> = Grid::from(input);

        self.generate_history(&grid).0.len().to_string()
    }

    fn part_two(&self, input: &str) -> String {
        let grid: Grid<char> = Grid::from(input);
        let guard = grid.get_first_position(&'^').unwrap();

        self.generate_history(&grid)
            .0
            .into_iter()
            .skip(1)
            .filter(|v| {
                if v == &guard {
                    return false;
                }

                let mut grid_with_obstacle = grid.clone();
                grid_with_obstacle.modify(*v, '#');

                self.generate_history(&grid_with_obstacle).1 == Reason::Loop
            })
            .count()
            .to_string()
    }
}

impl Day06 {
    fn generate_history(&self, grid: &Grid<char>) -> (Vec<Point>, Reason) {
        let obstructions = grid.get_all_positions(&'#');
        let guard = grid.get_first_position(&'^').unwrap();
        let surface = grid.surface_range();

        let mut guard = Vector::new(guard, North);
        let mut visited_positions: Vec<Vector> = Vec::new();

        let reason: Reason;

        loop {
            if !surface.contains(guard.position()) {
                reason = Reason::OutOfSurface;
                break;
            }

            // todo remove this visited_positions contains for part one, because it slows it down
            if visited_positions.contains(&guard) {
                reason = Reason::Loop;
                break;
            }

            visited_positions.push(guard);

            let next_position = guard.step();
            if obstructions.contains(&next_position.position()) {
                guard = guard.rotate_cw().step()
            } else {
                guard = next_position;
            }
        }

        (
            visited_positions
                .into_iter()
                .map(|v| v.position())
                .unique()
                .collect(),
            reason,
        )
    }
}

#[derive(PartialEq)]
enum Reason {
    OutOfSurface,
    Loop,
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

    #[test]
    fn part_two_example_test() {
        assert_eq!("6", Day06.part_two(EXAMPLE));
    }
}
