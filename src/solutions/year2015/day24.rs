use crate::solutions::Solution;
use itertools::Itertools;
use std::collections::HashSet;

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
    fn parse(&self, input: &str) -> Vec<u32> {
        input.lines().map(|x| x.parse().unwrap()).collect()
    }

    fn solve(&self, weights: &[u32]) -> u32 {
        let chunks = self.possible_chunks(weights);

        let mut candidates: Vec<Vec<u32>> = Vec::new();
        let mut number_of_packages: Option<usize> = None;

        for i in 0..chunks.len() {
            if self.can_be_in_first_group(&chunks, i, weights.len()) {
                let candidate = &chunks[i];

                if number_of_packages.is_none_or(|v| v == candidate.len()) {
                    number_of_packages = Some(i);
                    candidates.push(candidate.to_vec());
                }

                if number_of_packages.is_some_and(|v| v < candidate.len()) {
                    break;
                }
            }
        }

        candidates.iter().map(|c| c.iter().product()).min().unwrap()
    }

    fn can_be_in_first_group(&self, chunks: &[Vec<u32>], index: usize, weights_len: usize) -> bool {
        let a = &chunks[index];

        for i in (index + 1)..chunks.len() {
            let b = &chunks[i];

            for c in chunks.iter().skip(i + 1) {
                let mut set: HashSet<&u32> = HashSet::with_capacity(a.len() + b.len() + c.len());
                set.extend(a);
                set.extend(b);
                set.extend(c);

                if set.len() == weights_len {
                    return true;
                }
            }
        }

        false
    }

    fn possible_chunks(&self, weights: &[u32]) -> Vec<Vec<u32>> {
        let mut results = Vec::new();
        let sum: u32 = weights.iter().sum::<u32>() / 3;

        for chunk_length in 1..weights.len() - 1 {
            let combinations = weights.iter().combinations(chunk_length);

            for combination in combinations {
                let chunk_sum: u32 = combination.iter().copied().sum();

                if chunk_sum == sum {
                    results.push(combination.into_iter().copied().collect());
                }
            }
        }

        results
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: [u32; 10] = [1, 2, 3, 4, 5, 7, 8, 9, 10, 11];

    #[test]
    fn possible_chunks() {
        assert_eq!(25, Day24.possible_chunks(&EXAMPLE).len());
    }

    #[test]
    fn part_one_example() {
        assert_eq!(99, Day24.solve(&EXAMPLE));
    }
}
