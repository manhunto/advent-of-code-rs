use crate::solutions::Solution;
use crate::utils::grid::Grid;

pub struct Day04;

impl Solution for Day04 {
    fn part_one(&self, input: &str) -> String {
        const ROLL_OF_PAPER: char = '@';
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

    fn part_two(&self, _input: &str) -> String {
        String::from("0")
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
}
