use crate::solutions::Solution;
use itertools::Itertools;
use std::collections::{HashMap, VecDeque};

type PersonName = u8;
type Preferences = HashMap<(PersonName, PersonName), i64>;

pub struct Day13;

impl Solution for Day13 {
    fn part_one(&self, input: &str) -> String {
        let map = self.parse(input);
        let mut persons = self.persons(&map);
        let first_person = persons.pop_front().unwrap();
        let len = persons.len();

        persons
            .into_iter()
            .permutations(len)
            .map(|v| {
                let mut vec = Vec::from_iter([first_person]);
                vec.extend(v);

                let len = vec.len() as i64;

                (0..vec.len()).fold(0i64, |acc, i| {
                    let person = vec[i];
                    let left_index = (i as i64 - 1).rem_euclid(len);
                    let left_person = vec[left_index as usize];
                    let right_index = (i as i64 + 1).rem_euclid(len);
                    let right_person = vec[right_index as usize];

                    let left_happiness = map.get(&(*person, *left_person)).unwrap();
                    let right_happiness = map.get(&(*person, *right_person)).unwrap();

                    acc + left_happiness + right_happiness
                })
            })
            .max()
            .unwrap()
            .to_string()
    }

    fn part_two(&self, _input: &str) -> String {
        String::from("0")
    }
}

impl Day13 {
    fn parse(&self, input: &str) -> Preferences {
        input.lines().fold(
            HashMap::with_capacity(input.lines().count()),
            |mut map, line| {
                let parts = line
                    .trim_end_matches('.')
                    .split_ascii_whitespace()
                    .collect_vec();

                let happiness_value = parts[3].parse::<i64>().unwrap();
                let happiness = match parts[2] {
                    "lose" => -happiness_value,
                    "gain" => happiness_value,
                    _ => unimplemented!(),
                };

                let first_name = parts[0].as_bytes()[0];
                let second_name = parts[10].as_bytes()[0];

                map.insert((first_name, second_name), happiness);

                map
            },
        )
    }

    fn persons<'a>(&self, map: &'a Preferences) -> VecDeque<&'a PersonName> {
        map.keys().map(|(k, _)| k).unique().sorted().collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = r#"Alice would gain 54 happiness units by sitting next to Bob.
Alice would lose 79 happiness units by sitting next to Carol.
Alice would lose 2 happiness units by sitting next to David.
Bob would gain 83 happiness units by sitting next to Alice.
Bob would lose 7 happiness units by sitting next to Carol.
Bob would lose 63 happiness units by sitting next to David.
Carol would lose 62 happiness units by sitting next to Alice.
Carol would gain 60 happiness units by sitting next to Bob.
Carol would gain 55 happiness units by sitting next to David.
David would gain 46 happiness units by sitting next to Alice.
David would lose 7 happiness units by sitting next to Bob.
David would gain 41 happiness units by sitting next to Carol."#;

    #[test]
    fn part_one_example_test() {
        assert_eq!("330", Day13.part_one(EXAMPLE));
    }
}
