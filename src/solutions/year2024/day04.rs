use crate::solutions::Solution;
use crate::utils::grid::Grid;
use crate::utils::point::Point;
use crate::utils::vector::Vector;

pub struct Day04;

impl Solution for Day04 {
    fn part_one(&self, input: &str) -> String {
        let grid: Grid<char> = Grid::from(input);

        grid.get_all_positions(&'X')
            .into_iter()
            .flat_map(|point_x| self.filter_vectors_towards_char(&grid, &point_x, 'M'))
            .filter_map(|vector| self.make_step(&grid, vector, 'A'))
            .filter_map(|vector| self.make_step(&grid, vector, 'S'))
            .count()
            .to_string()
    }

    fn part_two(&self, _input: &str) -> String {
        String::from('0')
    }
}

impl Day04 {
    fn filter_vectors_towards_char(
        &self,
        grid: &Grid<char>,
        point: &Point,
        char: char,
    ) -> Vec<Vector> {
        point
            .adjacent_with_diagonal_vectors()
            .into_iter()
            .filter_map(|p_m| {
                if grid
                    .get_for_point(&p_m.position())
                    .is_some_and(|m| m == &char)
                {
                    return Some(p_m);
                }
                None
            })
            .collect()
    }

    fn make_step(&self, grid: &Grid<char>, vector: Vector, char: char) -> Option<Vector> {
        let step = vector.step();
        if grid
            .get_for_point(&step.position())
            .is_some_and(|c| c == &char)
        {
            return Some(step);
        }

        None
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
