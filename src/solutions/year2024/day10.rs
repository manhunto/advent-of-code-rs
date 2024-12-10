use crate::solutions::Solution;
use crate::utils::grid::Grid;
use crate::utils::point::Point;
use itertools::Itertools;

pub struct Day10;

impl Solution for Day10 {
    fn part_one(&self, input: &str) -> String {
        let grid: Grid<i16> = Grid::from_custom(input.trim(), |c| {
            c.to_digit(10).map(|x| x as i16).unwrap_or(-1)
        });

        let mut current = 0;
        let mut current_points: Vec<Point> = grid.get_all_positions(&current);

        while current < 9 {
            let next = current + 1;
            current_points = current_points
                .iter()
                .flat_map(|point| {
                    point.adjacent().into_iter().filter(|next_point| {
                        grid.get_for_point(next_point)
                            .is_some_and(|value| value == &next)
                    })
                })
                .unique()
                .collect();

            current += 1;
        }

        current_points.len().to_string()
    }

    fn part_two(&self, _input: &str) -> String {
        String::from('0')
    }
}

#[cfg(test)]
mod tests {
    use crate::solutions::year2024::day10::Day10;
    use crate::solutions::Solution;

    #[test]
    fn part_one_example_test() {
        const FIRST_EXAMPLE: &str = r#"0123
1234
8765
9876"#;
        assert_eq!("1", Day10.part_one(FIRST_EXAMPLE));

        const SECOND_EXAMPLE: &str = r#"...0...
...1...
...2...
6543456
7.....7
8.....8
9.....9"#;

        assert_eq!("2", Day10.part_one(SECOND_EXAMPLE));
    }
}
