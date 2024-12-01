use crate::solutions::Solution;

pub struct Day01;

impl Solution for Day01 {
    fn part_one(&self, input: &str) -> String {
        let (mut left, mut right): (Vec<i32>, Vec<i32>) = input
            .lines()
            .map(|line| {
                let mut split = line.split_terminator("   ");
                (
                    split.next().unwrap().parse::<i32>().unwrap(),
                    split.next().unwrap().parse::<i32>().unwrap(),
                )
            })
            .unzip();

        left.sort_unstable();
        right.sort_unstable();

        left.iter()
            .zip(right.iter())
            .map(|(a, b)| (a - b).abs())
            .sum::<i32>()
            .to_string()
    }

    fn part_two(&self, _input: &str) -> String {
        String::new()
    }
}

#[cfg(test)]
mod tests {
    use crate::solutions::year2024::day01::Day01;
    use crate::solutions::Solution;

    const EXAMPLE: &str = r#"3   4
4   3
2   5
1   3
3   9
3   3"#;

    #[test]
    fn part_one_example_test() {
        assert_eq!("11", Day01.part_one(EXAMPLE));
    }
}
