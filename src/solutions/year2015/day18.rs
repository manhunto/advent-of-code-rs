use crate::solutions::Solution;
use crate::utils::grid::Grid;
use crate::utils::point::Point;

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

    fn part_two(&self, input: &str) -> String {
        let mut grid: Grid<u8> = Grid::from_custom_as_bytes(input, |c| *c);
        let rows_range = grid.rows_range();
        let columns_range = grid.rows_range();
        let always_on = [
            Point::new(rows_range.start(), columns_range.start()),
            Point::new(rows_range.start(), columns_range.end()),
            Point::new(rows_range.end(), columns_range.end()),
            Point::new(rows_range.end(), columns_range.start()),
        ];

        for point in always_on {
            grid.modify(point, ON);
        }

        // todo extract light grid, without range calculations in constructor, just simple implementation on bytes
        for _ in 0..self.steps {
            grid = grid
                .iter()
                .map(|(point, c)| {
                    if always_on.contains(point) {
                        return (*point, *c);
                    }

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
