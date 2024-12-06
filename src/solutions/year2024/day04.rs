use crate::solutions::Solution;
use crate::utils::grid::Grid;
use crate::utils::point::Point;

const X: u8 = b'X';
const M: u8 = b'M';
const A: u8 = b'A';
const S: u8 = b'S';

pub struct Day04;

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
                self.has_pattern_on_diagonal(&grid, &a.north_west(), &a.south_east())
                    && self.has_pattern_on_diagonal(&grid, &a.north_east(), &a.south_west())
            })
            .count()
            .to_string()
    }
}

impl Day04 {
    fn has_pattern_on_diagonal(&self, grid: &Grid<u8>, p1: &Point, p2: &Point) -> bool {
        grid.is_for_point(p1, M) && grid.is_for_point(p2, S)
            || grid.is_for_point(p1, S) && grid.is_for_point(p2, M)
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
