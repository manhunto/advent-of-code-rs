use crate::solutions::Solution;
use md5::compute;

pub struct Day04;

impl Solution for Day04 {
    fn part_one(&self, input: &str) -> String {
        self.answer(input, "00000")
    }

    fn part_two(&self, input: &str) -> String {
        self.answer(input, "000000")
    }
}

impl Day04 {
    fn answer(&self, input: &str, starts_with: &str) -> String {
        (0u64..)
            .find(|answer| {
                let hash = format!("{}{}", input, answer);
                let digest = compute(hash);

                let x = format!("{:x}", digest);

                x.starts_with(starts_with)
            })
            .unwrap()
            .to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_one_example_test() {
        assert_eq!("609043", Day04.part_one("abcdef"));
        assert_eq!("1048970", Day04.part_one("pqrstuv"));
    }
}
