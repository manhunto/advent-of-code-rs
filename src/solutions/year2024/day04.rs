use crate::solutions::Solution;
use crate::utils::grid::Grid;

pub struct Day04;

const X: u8 = b'X';
const M: u8 = b'M';
const A: u8 = b'A';
const S: u8 = b'S';

impl Solution for Day04 {
    fn part_one(&self, input: &str) -> String {
        let grid: Grid<u8> = Grid::from_custom(input, |c| c as u8);

        grid.get_all_positions(&X)
            .into_iter()
            .flat_map(|x| x.adjacent_with_diagonal_vectors())
            .filter(|vector| grid.is_for_point(&vector.position(), M))
            .map(|m| m.step())
            .filter(|vector| grid.is_for_point(&vector.position(), A))
            .map(|m| m.step())
            .filter(|vector| grid.is_for_point(&vector.position(), S))
            .count()
            .to_string()
    }

    fn part_two(&self, input: &str) -> String {
        let grid: Grid<u8> = Grid::from_custom(input, |c| c as u8);

        grid.get_all_positions(&A)
            .into_iter()
            .filter(|a| {
                let mut x = 0;
                if grid.is_for_point(&a.north_west(), M) && grid.is_for_point(&a.south_east(), S)
                    || grid.is_for_point(&a.north_west(), S)
                        && grid.is_for_point(&a.south_east(), M)
                {
                    x += 1;
                }

                if grid.is_for_point(&a.north_east(), M) && grid.is_for_point(&a.south_west(), S)
                    || grid.is_for_point(&a.north_east(), S)
                        && grid.is_for_point(&a.south_west(), M)
                {
                    x += 1;
                }

                x == 2
            })
            .count()
            .to_string()
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

    #[test]
    fn part_two_example_test() {
        assert_eq!("9", Day04.part_two(EXAMPLE));
    }
}
