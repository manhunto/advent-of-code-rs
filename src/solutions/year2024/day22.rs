use crate::solutions::Solution;
use itertools::Itertools;
use std::collections::{HashMap, HashSet};

pub struct Day22;

const SECRET_COUNT: usize = 2000;

impl Solution for Day22 {
    fn part_one(&self, input: &str) -> String {
        input
            .lines()
            .map(|line| line.parse::<usize>().unwrap())
            .map(|initial| {
                let secrets = self.next_secrets(initial, SECRET_COUNT);

                *secrets.last().unwrap()
            })
            .sum::<usize>()
            .to_string()
    }

    fn part_two(&self, input: &str) -> String {
        let mut map: HashMap<[i8; 4], usize> = HashMap::new();

        input.lines().for_each(|line| {
            let mut seen_map: HashSet<[i8; 4]> = HashSet::new();
            let initial: usize = line.parse().unwrap();
            let secrets = self.next_secrets(initial, SECRET_COUNT);
            let prices = self.prices(secrets);
            let diffs = self.diffs(&prices);

            for i in 4..diffs.len() {
                let key = [diffs[i - 4], diffs[i - 3], diffs[i - 2], diffs[i - 1]];

                if seen_map.insert(key) {
                    *map.entry(key).or_insert(0) += prices[i] as usize;
                }
            }
        });

        map.values().max().unwrap().to_string()
    }
}

impl Day22 {
    fn next_secrets(&self, initial: usize, number_of_secrets: usize) -> Vec<usize> {
        let mut secret = initial;
        let mut next_secrets = vec![initial];

        for _ in 0..number_of_secrets {
            secret = self.mix_and_prune(secret, |s| s * 64);
            secret = self.mix_and_prune(secret, |s| s / 32);
            secret = self.mix_and_prune(secret, |s| s * 2048);

            next_secrets.push(secret);
        }

        next_secrets
    }

    fn mix_and_prune(&self, current: usize, calculations: fn(usize) -> usize) -> usize {
        (current ^ calculations(current)) % 16777216
    }

    fn prices(&self, secrets: Vec<usize>) -> Vec<i8> {
        secrets.iter().map(|secret| (secret % 10) as i8).collect()
    }

    fn diffs(&self, secrets: &[i8]) -> Vec<i8> {
        secrets.iter().tuple_windows().map(|(a, b)| b - a).collect()
    }
}

#[cfg(test)]
mod tests {
    use crate::solutions::year2024::day22::Day22;
    use crate::solutions::Solution;

    const PART_ONE_EXAMPLE: &str = r#"1
10
100
2024"#;

    #[test]
    fn part_one_example() {
        assert_eq!("37327623", Day22.part_one(PART_ONE_EXAMPLE));
    }

    const PART_TWO_EXAMPLE: &str = r#"1
2
3
2024"#;

    #[test]
    fn part_two_example() {
        assert_eq!("23", Day22.part_two(PART_TWO_EXAMPLE));
    }

    #[test]
    fn next_secrets() {
        let secrets = Day22.next_secrets(123, 10);
        let mut iter = secrets.iter();

        assert_eq!(Some(&123), iter.next());
        assert_eq!(Some(&15887950), iter.next());
        assert_eq!(Some(&16495136), iter.next());
        assert_eq!(Some(&527345), iter.next());
        assert_eq!(Some(&704524), iter.next());
        assert_eq!(Some(&1553684), iter.next());
        assert_eq!(Some(&12683156), iter.next());
        assert_eq!(Some(&11100544), iter.next());
        assert_eq!(Some(&12249484), iter.next());
        assert_eq!(Some(&7753432), iter.next());
        assert_eq!(Some(&5908254), iter.next());
        assert_eq!(None, iter.next());
    }

    #[test]
    fn prices_and_diffs() {
        let secrets = Day22.next_secrets(123, 9);
        let prices = Day22.prices(secrets);
        assert_eq!(vec![3, 0, 6, 5, 4, 4, 6, 4, 4, 2], prices);

        let diffs = Day22.diffs(&prices);
        assert_eq!(vec![-3, 6, -1, -1, 0, 2, -2, 0, -2], diffs);
    }
}
