use crate::solutions::Solution;

pub struct Day01;

impl Solution for Day01 {
    fn part_one(&self, input: &str) -> String {
        input
            .bytes()
            .map(|b| match b {
                b'(' => 1,
                b')' => -1,
                _ => 0,
            })
            .sum::<isize>()
            .to_string()
    }

    fn part_two(&self, input: &str) -> String {
        let mut current_flor = 0;

        for (i, b) in input.bytes().enumerate() {
            current_flor += match b {
                b'(' => 1,
                b')' => -1,
                _ => 0,
            };

            if current_flor == -1 {
                return (i + 1).to_string();
            }
        }

        unreachable!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_one_example_test() {
        assert_eq!("0", Day01.part_one("(())"));
        assert_eq!("0", Day01.part_one("()()"));
        assert_eq!("3", Day01.part_one("((("));
        assert_eq!("3", Day01.part_one("(()(()("));
        assert_eq!("3", Day01.part_one("))((((("));
        assert_eq!("-1", Day01.part_one("())"));
        assert_eq!("-1", Day01.part_one("))("));
        assert_eq!("-3", Day01.part_one(")))"));
        assert_eq!("-3", Day01.part_one(")())())"));
    }

    #[test]
    fn part_two_example_test() {
        assert_eq!("1", Day01.part_two(")"));
        assert_eq!("5", Day01.part_two("()())"));
    }
}
