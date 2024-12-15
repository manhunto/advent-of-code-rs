use crate::solutions::Solution;
use crate::utils::direction::Direction;
use crate::utils::grid::Grid;

pub struct Day15;

impl Solution for Day15 {
    fn part_one(&self, input: &str) -> String {
        let (grid, directions) = self.parse(input);

        println!("{}", grid);
        println!("{:?}", directions);

        String::from('0')
    }

    fn part_two(&self, _input: &str) -> String {
        String::from('0')
    }
}

impl Day15 {
    fn parse(&self, input: &str) -> (Grid<char>, Vec<Direction>) {
        input
            .split_once("\n\n")
            .map(|(grid, directions)| {
                (
                    Grid::<char>::from(grid),
                    directions
                        .chars()
                        .map(|c| match c {
                            '^' => Direction::North,
                            '>' => Direction::East,
                            '<' => Direction::West,
                            'v' => Direction::South,
                            _ => unreachable!(),
                        })
                        .collect(),
                )
            })
            .unwrap()
    }
}

#[cfg(test)]
mod tests {
    use crate::solutions::year2024::day15::Day15;
    use crate::solutions::Solution;

    const SMALL_EXAMPLE: &str = r#"########
#..O.O.#
##@.O..#
#...O..#
#.#.O..#
#...O..#
#......#
########

<^^>>>vv<v>>v<<"#;

    #[test]
    fn part_one_small_example_test() {
        assert_eq!("0", Day15.part_one(SMALL_EXAMPLE));
    }
}
