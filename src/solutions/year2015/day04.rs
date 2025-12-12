use crate::solutions::Solution;
use md5::compute;

pub struct Day04;

impl Solution for Day04 {
    fn part_one(&self, input: &str) -> String {
        for answer in 0u64.. {
            // println!("{}", answer);
            let hash = format!("{}{}", input, answer);
            let digest = compute(hash);

            let x = format!("{:x}", digest);
            // println!("{}", x);
            if &x[0..5] == "00000" {
                return answer.to_string();
            }
        }

        unreachable!();
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
        assert_eq!("609043", Day04.part_one("abcdef"));
        assert_eq!("1048970", Day04.part_one("pqrstuv"));
    }
}
