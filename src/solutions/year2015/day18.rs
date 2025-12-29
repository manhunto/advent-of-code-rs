use crate::solutions::Solution;
use crate::utils::light_grid::LightGrid;

const ON: u8 = b'#';
const OFF: u8 = b'.';

pub struct Day18 {
    steps: usize,
}

impl Solution for Day18 {
    fn part_one(&self, input: &str) -> String {
        let mut grid = LightGrid::from_str_with(input, |c| c);

        for _ in 0..self.steps {
            grid = self.simulate_step(&grid, None);
        }

        grid.count_equal(&ON).to_string()
    }

    fn part_two(&self, input: &str) -> String {
        let mut grid = LightGrid::from_str_with(input, |c| c);
        let corners = self.get_corner_positions(&grid);

        for &(x, y) in &corners {
            grid.set(x, y, ON);
        }

        for _ in 0..self.steps {
            grid = self.simulate_step(&grid, Some(&corners));
        }

        grid.count_equal(&ON).to_string()
    }
}

impl Day18 {
    fn simulate_step(
        &self,
        grid: &LightGrid<u8>,
        stuck_corners: Option<&[(usize, usize)]>,
    ) -> LightGrid<u8> {
        grid.map(|x, y, &c| {
            if let Some(corners) = stuck_corners {
                if corners.iter().any(|&(cx, cy)| cx == x && cy == y) {
                    return ON;
                }
            }

            let adjacent_on = self.count_adjacent_lights(grid, x, y);
            self.next_light_state(c, adjacent_on)
        })
    }

    #[inline]
    fn count_adjacent_lights(&self, grid: &LightGrid<u8>, x: usize, y: usize) -> usize {
        grid.adjacent_with_diagonals(x, y)
            .iter()
            .filter(|&&(nx, ny)| grid.get(nx, ny) == Some(&ON))
            .count()
    }

    #[inline]
    fn next_light_state(&self, current: u8, adjacent_on: usize) -> u8 {
        match (current, adjacent_on) {
            (ON, 2..=3) => ON,
            (OFF, 3) => ON,
            _ => OFF,
        }
    }

    #[inline]
    fn get_corner_positions(&self, grid: &LightGrid<u8>) -> [(usize, usize); 4] {
        let max_x = grid.width() - 1;
        let max_y = grid.height() - 1;

        [(0, 0), (0, max_y), (max_x, 0), (max_x, max_y)]
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
        let day = Day18 { steps: 4 };
        assert_eq!("4", day.part_one(EXAMPLE));
    }

    #[test]
    fn part_two_example() {
        let day = Day18 { steps: 5 };
        assert_eq!("17", day.part_two(EXAMPLE));
    }
}
