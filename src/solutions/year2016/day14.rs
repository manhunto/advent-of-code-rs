use crate::solutions::Solution;
use md5::compute;
use std::cell::RefCell;
use std::collections::HashMap;

pub struct Day14;

impl Solution for Day14 {
    fn part_one(&self, input: &str) -> String {
        let input = input.trim();
        let hash_generator = HashGenerator::new(input.to_string());
        let mut count = 0;

        for i in 0usize.. {
            let three_hash = hash_generator.get(i);
            let three_tuples = self.find_all_tuples(&three_hash, 3);

            if !three_tuples.is_empty() {
                let first_three_tuple = three_tuples.first().unwrap();

                for j in i + 1..=i + 1000 {
                    let five_hash = hash_generator.get(j);
                    let five_tuples = self.find_all_tuples(&five_hash, 5);

                    if five_tuples.contains(first_three_tuple) {
                        count += 1;

                        if count == 64 {
                            return i.to_string();
                        }

                        break;
                    }
                }
            }
        }

        unreachable!();
    }

    fn part_two(&self, _input: &str) -> String {
        String::from("0")
    }
}

struct HashGenerator {
    prefix: String,
    hashes: RefCell<HashMap<usize, String>>,
}

impl HashGenerator {
    fn new(prefix: String) -> HashGenerator {
        Self {
            prefix,
            hashes: RefCell::new(HashMap::new()),
        }
    }

    fn get(&self, i: usize) -> String {
        if let Some(cached) = self.hashes.borrow().get(&i) {
            return cached.clone();
        }

        let hash = format!("{}{}", self.prefix, i);
        let digest = compute(hash);
        let hex = format!("{:x}", digest);

        self.hashes.borrow_mut().insert(i, hex.clone());

        hex
    }
}

impl Day14 {
    fn find_all_tuples(&self, hex: &str, length: usize) -> Vec<char> {
        let mut current: Vec<char> = Vec::new();

        while let Some(tuples) = self.find_next_tuple(hex, length, &current) {
            current.push(tuples);
        }

        current
    }

    fn find_next_tuple(&self, hex: &str, length: usize, skip: &[char]) -> Option<char> {
        let mut iter = hex.chars();
        let current = iter.find(|c| !skip.contains(c));

        current?;

        let mut current = current.unwrap();
        let mut count = 1;

        for c in iter {
            if c == current && !skip.contains(&c) {
                count += 1;

                if count == length {
                    return Some(c);
                }
            } else {
                current = c;
                count = 1;
            }
        }

        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "abc";

    #[test]
    fn part_one_example() {
        assert_eq!("22728", Day14.part_one(EXAMPLE));
    }

    #[test]
    fn find_all_tuples() {
        assert_eq!(vec!['8'], Day14.find_all_tuples("cc38887a5", 3));
        assert_eq!(vec!['8', 'a'], Day14.find_all_tuples("cc38887aaa5", 3));
        assert_eq!(vec!['a'], Day14.find_all_tuples("aaa", 3));
    }
}
