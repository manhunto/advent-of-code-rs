use crate::solutions::Solution;
use crate::utils::grid::Grid;
use crate::utils::light_grid::LightGrid;
use crate::utils::point::Point;

const ON: u8 = b'#';
const OFF: u8 = b'.';

pub struct Day18 {
    steps: usize,
}

impl Solution for Day18 {
    fn part_one(&self, input: &str) -> String {
        let mut grid = LightGrid::from_str_with(input, |c| c);

        for _ in 0..self.steps {
            grid = grid.map(|x, y, c| {
                let adjacent_lights_on = self.count_adjacent_lights(&grid, x, y);

                self.determine_light(*c, adjacent_lights_on)
            });
        }

        grid.count_equal(&ON).to_string()
    }

    fn part_two(&self, input: &str) -> String {
        let mut grid = LightGrid::from_str_with(input, |c| c);

        let always_on = [
            (0, 0),
            (0, grid.height() - 1),
            (grid.width() - 1, grid.height() - 1),
            (grid.width() - 1, 0),
        ];

        for point in always_on {
            grid.set(point.0, point.1, ON);
        }

        for _ in 0..self.steps {
            grid = grid.map(|x, y, c| {
                if always_on.contains(&(x, y)) {
                    return *c;
                }

                let adjacent_lights_on = self.count_adjacent_lights(&grid, x, y);

                self.determine_light(*c, adjacent_lights_on)
            });
        }

        grid.count_equal(&ON).to_string()
    }
}

impl Day18 {
    fn count_adjacent_lights(&self, grid: &LightGrid<u8>, x: usize, y: usize) -> usize {
        grid.adjacent_with_diagonals(x, y)
            .iter()
            .filter(|(nx, ny)| grid.get(*nx, *ny) == Some(&b'#'))
            .count()
    }

    fn determine_light(&self, current: u8, adjacent_lights_on: usize) -> u8 {
        match current {
            ON if adjacent_lights_on == 2 || adjacent_lights_on == 3 => ON,
            OFF if adjacent_lights_on == 3 => ON,
            _ => OFF,
        }
    }
}

impl Default for Day18 {
    fn default() -> Self {
        Self { steps: 100 }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = r#".#.#.#
...##.
#....#
..#...
#.#..#
####.."#;

    #[test]
    fn part_one_example() {
        assert_eq!("4", day_part_one().part_one(EXAMPLE));
    }

    #[test]
    fn part_two_example() {
        assert_eq!("17", day_part_two().part_two(EXAMPLE));
    }

    fn day_part_one() -> Day18 {
        Day18 { steps: 4 }
    }

    fn day_part_two() -> Day18 {
        Day18 { steps: 5 }
    }
}
