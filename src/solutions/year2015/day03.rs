use crate::solutions::Solution;
use crate::utils::point::Point;
use std::collections::HashMap;

pub struct Day03;

impl Solution for Day03 {
    fn part_one(&self, input: &str) -> String {
        let mut visited_houses: HashMap<Point, u64> = HashMap::new();
        let mut current = Point::new(0, 0);

        visited_houses.insert(current, 1);

        for b in input.bytes() {
            current = match b {
                b'>' => current.east(),
                b'<' => current.west(),
                b'^' => current.north(),
                b'v' => current.south(),
                _ => unreachable!(),
            };
            visited_houses
                .entry(current)
                .and_modify(|e| *e += 1)
                .or_insert(1);
        }

        visited_houses.keys().count().to_string()
    }

    fn part_two(&self, _input: &str) -> String {
        String::from("0")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_one_example_test() {
        assert_eq!("2", Day03.part_one(">"));
        assert_eq!("4", Day03.part_one("^>v<"));
        assert_eq!("2", Day03.part_one("^v^v^v^v^v"));
    }
}
