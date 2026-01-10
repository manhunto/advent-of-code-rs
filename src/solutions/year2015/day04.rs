use crate::solutions::Solution;
use crate::utils::crypto::md5::DigestExt;
use md5::{compute, Digest};

pub struct Day04;

impl Solution for Day04 {
    fn part_one(&self, input: &str) -> String {
        self.answer(input, |digest| digest.starts_with_five_zeros())
    }

    fn part_two(&self, input: &str) -> String {
        self.answer(input, |digest| digest.starts_with_six_zeros())
    }
}

impl Day04 {
    fn answer<F>(&self, input: &str, check_func: F) -> String
    where
        F: Fn(Digest) -> bool,
    {
        (0u64..)
            .find(|answer| {
                let hash = format!("{}{}", input, answer);
                let digest = compute(hash);

                check_func(digest)
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
