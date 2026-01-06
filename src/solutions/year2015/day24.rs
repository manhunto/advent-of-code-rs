use crate::solutions::Solution;
use itertools::Itertools;

pub struct Day24;

impl Solution for Day24 {
    fn part_one(&self, input: &str) -> String {
        let weights = self.parse(input);

        self.solve(&weights).to_string()
    }

    fn part_two(&self, _input: &str) -> String {
        String::from("0")
    }
}

impl Day24 {
    fn parse(&self, input: &str) -> Vec<u64> {
        input.lines().map(|x| x.parse().unwrap()).collect()
    }

    fn solve(&self, weights: &[u64]) -> u64 {
        let sum: u64 = weights.iter().sum::<u64>() / 3;
        let max_chunk_length = weights.len() - 1;

        (1..max_chunk_length)
            .map(|chunk_length| {
                weights
                    .iter()
                    .combinations(chunk_length)
                    .filter(|a| a.iter().map(|x| **x).sum::<u64>() == sum)
                    .filter(|candidate| {
                        let mut left_to_share = weights.to_vec();
                        left_to_share.retain(|v| !candidate.contains(&v));

                        let is_valid = (candidate.len()..max_chunk_length).any(|b_chunk_length| {
                            left_to_share
                                .clone()
                                .into_iter()
                                .combinations(b_chunk_length)
                                .filter(|b| b.iter().sum::<u64>() == sum)
                                .any(|b| {
                                    let last_chunk_weight = left_to_share
                                        .iter()
                                        .filter(|v| !b.contains(v))
                                        .sum::<u64>();

                                    last_chunk_weight == sum
                                })
                        });

                        is_valid
                    })
                    .collect_vec()
            })
            .find(|candidates| !candidates.is_empty())
            .unwrap()
            .into_iter()
            .map(|v| v.into_iter().product::<u64>())
            .min()
            .unwrap_or(0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: [u64; 10] = [1, 2, 3, 4, 5, 7, 8, 9, 10, 11];

    #[test]
    fn part_one_example() {
        assert_eq!(99, Day24.solve(&EXAMPLE));
    }

    const MY_EXAMPLE: [u64; 5] = [1, 7, 2, 6, 8];

    #[test]
    fn my_example() {
        assert_eq!(8, Day24.solve(&MY_EXAMPLE));
    }
}
