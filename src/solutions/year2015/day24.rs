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
            .find_map(|chunk_length| {
                weights
                    .iter()
                    .combinations(chunk_length)
                    .filter(|a| a.iter().map(|x| **x).sum::<u64>() == sum)
                    .map(|c| (c.iter().map(|x| **x).product::<u64>(), c))
                    .sorted_by(|a, b| a.0.cmp(&b.0))
                    .find(|(_, candidate)| {
                        let mut left_to_share = weights.to_vec();
                        left_to_share.retain(|v| !candidate.contains(&v));

                        let is_valid = (1..left_to_share.len()).any(|b_chunk_length| {
                            left_to_share
                                .iter()
                                .combinations(b_chunk_length)
                                .any(|b| b.iter().map(|x| **x).sum::<u64>() == sum)
                        });

                        is_valid
                    })
                    .map(|(qe, _)| qe)
            })
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
    fn part_one_my_example() {
        assert_eq!(8, Day24.solve(&MY_EXAMPLE));
    }
}
