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

    fn part_two(&self, _input: &str) -> String {
        String::from("0")
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
}
