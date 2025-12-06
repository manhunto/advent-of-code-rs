use crate::solutions::Solution;
use crate::utils::grid::Grid;
use crate::utils::point::Point;
use itertools::Itertools;

const ROLL_OF_PAPER: char = '@';
const REMOVED: char = 'X';

pub struct Day04;

impl Solution for Day04 {
    fn part_one(&self, input: &str) -> String {
        let grid: Grid<char> = Grid::from(input);

        grid.get_all_positions(&ROLL_OF_PAPER)
            .iter()
            .filter(|p| {
                p.adjacent_with_diagonal_vectors()
                    .iter()
                    .filter(|adj| grid.is_for_point(&adj.position(), ROLL_OF_PAPER))
                    .count()
                    < 4
            })
            .count()
            .to_string()
    }

    fn part_two(&self, input: &str) -> String {
        let mut grid: Grid<char> = Grid::from(input);

        grid.get_all_positions(&ROLL_OF_PAPER)
            .iter()
            .fold(0, |acc, roll| acc + Self::try_to_remove(&mut grid, roll))
            .to_string()
    }
}

impl Day04 {
    fn try_to_remove(grid: &mut Grid<char>, roll: &Point) -> u32 {
        let mut removed_count = 0u32;
        if grid
            .get_for_point(roll)
            .is_some_and(|e| *e != ROLL_OF_PAPER)
        {
            return removed_count;
        }

        let adjacent = roll.adjacent_with_diagonal_vectors();
        let adjacent_rolls = adjacent
            .iter()
            .filter(|adj| grid.is_for_point(&adj.position(), ROLL_OF_PAPER))
            .collect_vec();

        if adjacent_rolls.len() < 4 {
            grid.modify(*roll, REMOVED);
            removed_count += 1;

            for adj_roll in adjacent_rolls {
                removed_count += Self::try_to_remove(grid, &adj_roll.position());
            }
        }

        removed_count
    }
}

#[cfg(test)]
mod tests {
    use crate::solutions::year2025::day04::Day04;
    use crate::solutions::Solution;

    const EXAMPLE: &str = r#"..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@."#;

    #[test]
    fn part_one_example_test() {
        assert_eq!("13", Day04.part_one(EXAMPLE));
    }

    #[test]
    fn part_two_example_test() {
        assert_eq!("43", Day04.part_two(EXAMPLE));
    }
}
