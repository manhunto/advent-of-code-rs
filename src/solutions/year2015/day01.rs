use crate::solutions::Solution;

const BASEMENT: isize = -1;

pub struct Day01;

impl Solution for Day01 {
    fn part_one(&self, input: &str) -> String {
        input
            .bytes()
            .map(Day01::map_byte)
            .sum::<isize>()
            .to_string()
    }

    fn part_two(&self, input: &str) -> String {
        input
            .bytes()
            .map(Day01::map_byte)
            .scan(0, |floor, change| {
                *floor += change;

                Some(*floor)
            })
            .position(|floor| floor == BASEMENT)
            .map(|i| i + 1)
            .unwrap()
            .to_string()
    }
}

impl Day01 {
    fn map_byte(b: u8) -> isize {
        match b {
            b'(' => 1,
            b')' => -1,
            _ => 0,
        }
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
