use crate::solutions::Solution;

pub struct Day06;

impl Solution for Day06 {
    fn part_one(&self, _input: &str) -> String {
        String::from('0')
    }

    fn part_two(&self, _input: &str) -> String {
        String::from('0')
    }
}

#[cfg(test)]
mod tests {
    use crate::solutions::year2024::day06::Day06;
    use crate::solutions::Solution;

    const EXAMPLE: &str = r#"....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#..."#;

    #[test]
    #[ignore]
    fn part_one_example_test() {
        assert_eq!("41", Day06.part_one(EXAMPLE));
    }
}
