use crate::solutions::Solution;
use crate::utils::crypto::md5::DigestExt;
use itertools::FoldWhile::{Continue, Done};
use itertools::Itertools;
use md5::compute;

pub struct Day05;

impl Solution for Day05 {
    fn part_one(&self, input: &str) -> String {
        self.iter(input.trim())
            .take(8)
            .map(|str| str.chars().nth(5).unwrap())
            .collect::<String>()
    }

    fn part_two(&self, input: &str) -> String {
        self.iter(input.trim())
            .fold_while([None; 8], |mut acc, str| {
                let position = &str[5..6];

                if let Ok(pos) = position.parse::<usize>() {
                    if pos < acc.len() && acc[pos].is_none() {
                        acc[pos] = Some(str.chars().nth(6).unwrap());
                    }
                }

                if acc.iter().any(|x| x.is_none()) {
                    return Continue(acc);
                }

                Done(acc)
            })
            .into_inner()
            .iter()
            .map(|x| x.unwrap())
            .collect()
    }
}

impl Day05 {
    fn iter<'a>(&'a self, door_id: &'a str) -> impl Iterator<Item = String> + 'a {
        let door_id_bytes = door_id.as_bytes();

        (0u64..)
            .map(move |n| {
                let mut input = door_id_bytes.to_vec();
                input.extend_from_slice(n.to_string().as_bytes());
                compute(&input)
            })
            .filter(|digest| digest.starts_with_five_zeros())
            .map(|digest| format!("{:x}", digest))
    }
}

#[cfg(test)]
mod tests {
    // use super::*;

    // todo: it is very slow
    // #[test]
    // fn part_one_example() {
    //     assert_eq!("18f47a30", Day05.part_one("abc"));
    // }
    //
    // #[test]
    // fn part_two_example() {
    //     assert_eq!("05ace8e3", Day05.part_two("abc"));
    // }
}
