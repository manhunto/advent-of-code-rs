use crate::solutions::Solution;
use crate::utils::grid::Grid;

pub struct Day12;

impl Solution for Day12 {
    fn part_one(&self, input: &str) -> String {
        Grid::<char>::from(input)
            .get_all_regions()
            .iter()
            .map(|filled_region| filled_region.perimeter() * filled_region.area())
            .sum::<usize>()
            .to_string()
    }

    fn part_two(&self, input: &str) -> String {
        Grid::<char>::from(input)
            .get_all_regions()
            .iter()
            .map(|filled_region| filled_region.corners() * filled_region.area())
            .sum::<usize>()
            .to_string()
    }
}

#[cfg(test)]
mod tests {
    use crate::solutions::year2024::day12::Day12;
    use crate::solutions::Solution;

    const EXAMPLE_1: &str = r#"AAAA
BBCD
BBCC
EEEC"#;

    #[test]
    fn part_one_example_1() {
        let result = (10 * 4) + (10 * 4) + (8 * 4) + (3 * 8) + 4;

        assert_eq!(result.to_string(), Day12.part_one(EXAMPLE_1));
    }

    #[test]
    fn part_two_example_1() {
        assert_eq!("80", Day12.part_two(EXAMPLE_1));
    }

    const EXAMPLE_2: &str = r#"OOOOO
OXOXO
OOOOO
OXOXO
OOOOO"#;

    #[test]
    fn part_one_example_2() {
        let result = 4 * 4 + (21 * 36);

        assert_eq!(result.to_string(), Day12.part_one(EXAMPLE_2));
    }

    #[test]
    fn part_two_example_2() {
        assert_eq!("436", Day12.part_two(EXAMPLE_2));
    }

    const EXAMPLE_3: &str = r#"RRRRIICCFF
RRRRIICCCF
VVRRRCCFFF
VVRCCCJFFF
VVVVCJJCFE
VVIVCCJJEE
VVIIICJJEE
MIIIIIJJEE
MIIISIJEEE
MMMISSJEEE"#;

    #[test]
    fn part_one_example_3() {
        assert_eq!("1930", Day12.part_one(EXAMPLE_3));
    }

    #[test]
    fn part_two_example_3() {
        assert_eq!("1206", Day12.part_two(EXAMPLE_3));
    }

    #[test]
    fn part_two_e_shape() {
        const EXAMPLE: &str = r#"EEEEE
EXXXX
EEEEE
EXXXX
EEEEE"#;

        assert_eq!("236", Day12.part_two(EXAMPLE));
    }

    #[test]
    fn part_two_last_example() {
        const EXAMPLE: &str = r#"AAAAAA
AAABBA
AAABBA
ABBAAA
ABBAAA
AAAAAA"#;

        assert_eq!("368", Day12.part_two(EXAMPLE));
    }
}
