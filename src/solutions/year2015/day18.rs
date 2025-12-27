use crate::solutions::Solution;
use crate::utils::grid::Grid;

const ON: u8 = b'#';
const OFF: u8 = b'.';

pub struct Day18 {
    steps: usize,
}

impl Solution for Day18 {
    fn part_one(&self, input: &str) -> String {
        let mut grid: Grid<u8> = Grid::from_custom_as_bytes(input, |c| *c);

        // todo extract light grid, without range calculations in constructor, just simple implementation on bytes
        for _ in 0..self.steps {
            grid = grid
                .iter()
                .map(|(point, c)| {
                    let adjacent_lights_on = point
                        .adjacent_with_diagonals()
                        .iter()
                        .filter(|p| grid.is_for_point(p, ON))
                        .count();

                    let new_c = match *c {
                        ON if adjacent_lights_on == 2 || adjacent_lights_on == 3 => ON,
                        OFF if adjacent_lights_on == 3 => ON,
                        _ => OFF,
                    };

                    (*point, new_c)
                })
                .collect();
        }

        grid.get_all_positions(&ON).len().to_string()
    }

    fn part_two(&self, _input: &str) -> String {
        String::from("0")
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
        assert_eq!("4", day().part_one(EXAMPLE));
    }

    fn day() -> Day18 {
        Day18 { steps: 4 }
    }
}
