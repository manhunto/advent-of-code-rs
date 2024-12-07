use crate::solutions::Solution;
use crate::utils::direction::Direction::North;
use crate::utils::grid::Grid;
use crate::utils::point::Point;
use crate::utils::surface_range::SurfaceRange;
use crate::utils::vector::Vector;
use std::collections::HashSet;

pub struct Day06;

const OBSTRUCTION: char = '#';
const STARTING_POSITION: char = '^';

impl Solution for Day06 {
    fn part_one(&self, input: &str) -> String {
        let grid: Grid<char> = Grid::from(input);

        let obstructions = grid.get_all_positions(&OBSTRUCTION);
        let guard = grid.get_first_position(&STARTING_POSITION).unwrap();
        let surface = grid.surface_range();

        let mut guard = Vector::new(guard, North);
        let mut visited_positions: HashSet<Point> = HashSet::new();

        while surface.contains(guard.position()) {
            visited_positions.insert(guard.position());

            guard = self.next_step(guard, &obstructions);
        }

        visited_positions.len().to_string()
    }

    fn part_two(&self, input: &str) -> String {
        let grid: Grid<char> = Grid::from(input);

        let obstructions = grid.get_all_positions(&OBSTRUCTION);
        let guard = grid.get_first_position(&STARTING_POSITION).unwrap();
        let surface = grid.surface_range();

        let mut guard = Vector::new(guard, North);
        guard = self.next_step(guard, &obstructions); // skip initial position

        let mut loop_count: u32 = 0;
        while surface.contains(guard.position()) {
            if self.does_it_loop(guard, &obstructions, &surface) {
                loop_count += 1;
            }

            guard = self.next_step(guard, &obstructions);
        }

        loop_count.to_string()
    }
}

impl Day06 {
    fn next_step(&self, guard: Vector, obstructions: &[Point]) -> Vector {
        let next_position = guard.forward();
        if obstructions.contains(&next_position.position()) {
            return guard.rotate_cw().forward();
        }

        next_position
    }

    fn does_it_loop(&self, guard: Vector, obstructions: &[Point], surface: &SurfaceRange) -> bool {
        let mut visited_positions: Vec<Vector> = Vec::new();
        let mut obstructions = obstructions.to_owned();
        obstructions.push(guard.position());

        let mut guard = guard.backward();
        loop {
            visited_positions.push(guard);

            guard = self.next_step(guard, &obstructions);

            if visited_positions.contains(&guard) {
                return true;
            }

            if !surface.contains(guard.position()) {
                return false;
            }
        }
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

    #[test]
    fn part_two_example_test() {
        assert_eq!("6", Day06.part_two(EXAMPLE));
    }
}
