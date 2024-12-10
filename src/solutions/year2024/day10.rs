use crate::solutions::Solution;
use crate::utils::grid::Grid;
use itertools::Itertools;

pub struct Day10;

impl Solution for Day10 {
    fn part_one(&self, input: &str) -> String {
        let grid: Grid<i16> = Grid::from_custom(input.trim(), |c| {
            c.to_digit(10).map(|x| x as i16).unwrap_or(-1)
        });

        grid.get_all_positions(&0)
            .iter()
            .map(|p| {
                let mut current = 0;
                let mut current_points = vec![*p];

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

                    current = next;
                }

                current_points.len()
            })
            .sum::<usize>()
            .to_string()
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
    fn part_one_example_test_1() {
        const FIRST_EXAMPLE: &str = r#"0123
1234
8765
9876"#;
        assert_eq!("1", Day10.part_one(FIRST_EXAMPLE));
    }

    #[test]
    fn part_one_example_test_2() {
        const EXAMPLE: &str = r#"...0...
...1...
...2...
6543456
7.....7
8.....8
9.....9"#;

        assert_eq!("2", Day10.part_one(EXAMPLE));
    }

    #[test]
    fn part_one_example_test_3() {
        const EXAMPLE: &str = r#"..90..9
...1.98
...2..7
6543456
765.987
876....
987...."#;

        assert_eq!("4", Day10.part_one(EXAMPLE));
    }

    #[test]
    fn part_one_example_test_4() {
        const EXAMPLE: &str = r#"10..9..
2...8..
3...7..
4567654
...8..3
...9..2
.....01"#;

        assert_eq!("3", Day10.part_one(EXAMPLE));
    }

    #[test]
    fn part_one_example_test_5() {
        const EXAMPLE: &str = r#"89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732"#;

        assert_eq!("36", Day10.part_one(EXAMPLE));
    }
}
