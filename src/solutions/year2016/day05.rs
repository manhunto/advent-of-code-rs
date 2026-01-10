use crate::solutions::Solution;
use md5::compute;

pub struct Day05;

impl Solution for Day05 {
    fn part_one(&self, input: &str) -> String {
        let door_id = input.trim();

        (0u64..)
            .map(|n| {
                let hash = format!("{}{}", door_id, n);
                let digest = compute(hash);

                format!("{:x}", digest)
            })
            .filter(|x| x.starts_with("00000"))
            .take(8)
            .map(|str| str.chars().nth(5).unwrap())
            .collect::<String>()
    }

    fn part_two(&self, _input: &str) -> String {
        String::from("0")
    }
}

#[cfg(test)]
mod tests {
    // todo: it is very slow
    // use super::*;
    //
    // #[test]
    // fn part_one_example() {
    //     assert_eq!("18f47a30", Day05.part_one("abc"));
    // }
}
