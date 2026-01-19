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
        let three_tuples = Md5Tuple::new(3);
        let five_tuples = Md5Tuple::new(5);

        for i in 0usize.. {
            let three_hash = hash_generator.get(i);

            if let Some(first_three_tuple) = three_tuples.find_first_tuple(three_hash) {
                for j in i + 1..=i + 1000 {
                    let five_hash = hash_generator.get(j);

                    if five_tuples.contains_tuple(&five_hash, first_three_tuple) {
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

struct Md5Tuple {
    length: usize,
    first_tuple_cache: RefCell<HashMap<String, Option<u8>>>,
}

impl Md5Tuple {
    fn new(length: usize) -> Md5Tuple {
        Self {
            length,
            first_tuple_cache: RefCell::new(HashMap::new()),
        }
    }

    fn find_first_tuple(&self, digest: String) -> Option<u8> {
        if let Some(cached) = self.first_tuple_cache.borrow().get(&digest.clone()) {
            return *cached;
        }

        let result = {
            let str = digest.as_str();
            let mut iter = str.bytes();
            let mut current = iter.next().unwrap();

            let mut count = 1;

            for c in iter {
                if c == current {
                    count += 1;

                    if count == self.length {
                        return Some(c);
                    }
                } else {
                    current = c;
                    count = 1;
                }
            }

            None
        };

        self.first_tuple_cache.borrow_mut().insert(digest, result);

        result
    }

    fn contains_tuple(&self, digest: &str, char_byte: u8) -> bool {
        let mut count = 0;

        for &byte in digest.as_bytes() {
            if byte == char_byte {
                count += 1;
                if count == self.length {
                    return true;
                }
            } else {
                count = 0;
            }
        }

        false
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
        assert_eq!(
            b'8',
            Md5Tuple::new(3)
                .find_first_tuple("cc38887a5".to_string())
                .unwrap()
        );
        assert_eq!(
            b'8',
            Md5Tuple::new(3)
                .find_first_tuple("cc38887aaa5".to_string())
                .unwrap()
        );
        assert_eq!(
            b'a',
            Md5Tuple::new(3)
                .find_first_tuple("aaa".to_string())
                .unwrap()
        );
    }
}
