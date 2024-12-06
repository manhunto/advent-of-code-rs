use crate::solutions::Solution;
use crate::utils::grid::Grid;

pub struct Day04;

impl Solution for Day04 {
    fn part_one(&self, input: &str) -> String {
        let grid: Grid<char> = Grid::from(input);

        grid.get_all_positions(&'X')
            .into_iter()
            .flat_map(|x| x.adjacent_with_diagonal_vectors())
            .filter(|vector| grid.is_for_point(&vector.position(), 'M'))
            .map(|m| m.step())
            .filter(|vector| grid.is_for_point(&vector.position(), 'A'))
            .map(|m| m.step())
            .filter(|vector| grid.is_for_point(&vector.position(), 'S'))
            .count()
            .to_string()
    }

    fn part_two(&self, _input: &str) -> String {
        String::from('0')
    }
}

#[cfg(test)]
mod tests {
    use crate::solutions::year2024::day04::Day04;
    use crate::solutions::Solution;

    const EXAMPLE: &str = r#"MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX"#;

    #[test]
    fn part_one_example_test() {
        assert_eq!("18", Day04.part_one(EXAMPLE));
    }
}