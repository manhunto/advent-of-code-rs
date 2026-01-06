use crate::solutions::Solution;
use itertools::Itertools;

const GROUPS_P1: u64 = 3;
const GROUPS_P2: u64 = 4;

pub struct Day24;

impl Solution for Day24 {
    fn part_one(&self, input: &str) -> String {
        let weights = self.parse(input);

        self.solve(&weights, GROUPS_P1).to_string()
    }

    fn part_two(&self, input: &str) -> String {
        let weights = self.parse(input);

        self.solve(&weights, GROUPS_P2).to_string()
    }
}

impl Day24 {
    fn parse(&self, input: &str) -> Vec<u64> {
        input.lines().map(|x| x.parse().unwrap()).collect()
    }

    fn solve(&self, weights: &[u64], groups: u64) -> u64 {
        let target_sum: u64 = weights.iter().sum::<u64>() / groups;
        let max_chunk_length = weights.len() - 1;

        (1..max_chunk_length)
            .find_map(|chunk_length| {
                weights
                    .iter()
                    .combinations(chunk_length)
                    .filter(|a| a.iter().map(|x| **x).sum::<u64>() == target_sum)
                    .map(|c| (c.iter().map(|x| **x).product::<u64>(), c))
                    .sorted_by(|a, b| a.0.cmp(&b.0))
                    .find(|(_, candidate)| {
                        let mut left_to_share = weights.to_vec();
                        left_to_share.retain(|v| !candidate.contains(&v));

                        self.can_partition(&left_to_share, groups - 1, target_sum)
                    })
                    .map(|(qe, _)| qe)
            })
            .unwrap_or(0)
    }

    fn can_partition(&self, items: &[u64], remaining_groups: u64, target_sum: u64) -> bool {
        if remaining_groups == 1 {
            return true;
        }

        (1..items.len()).any(|chunk_length| {
            items
                .iter()
                .combinations(chunk_length)
                .filter(|candidate| candidate.iter().map(|x| **x).sum::<u64>() == target_sum)
                .any(|candidate| {
                    let mut remaining_items = items.to_vec();
                    remaining_items.retain(|v| !candidate.contains(&v));

                    self.can_partition(&remaining_items, remaining_groups - 1, target_sum)
                })
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: [u64; 10] = [1, 2, 3, 4, 5, 7, 8, 9, 10, 11];

    #[test]
    fn part_one_example() {
        assert_eq!(99, Day24.solve(&EXAMPLE, GROUPS_P1));
    }

    #[test]
    fn part_two_example() {
        assert_eq!(44, Day24.solve(&EXAMPLE, GROUPS_P2));
    }

    const MY_EXAMPLE: [u64; 5] = [1, 7, 2, 6, 8];

    #[test]
    fn part_one_my_example() {
        assert_eq!(8, Day24.solve(&MY_EXAMPLE, GROUPS_P1));
    }
}
