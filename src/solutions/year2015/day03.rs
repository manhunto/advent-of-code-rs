use crate::solutions::Solution;
use crate::utils::point::Point;
use std::collections::HashMap;

type Houses = HashMap<Point, u64>;

pub struct Day03;

impl Solution for Day03 {
    fn part_one(&self, input: &str) -> String {
        let mut visited_houses: Houses = HashMap::new();
        let mut santa = Point::new(0, 0);
        visited_houses.insert(santa, 1);

        for b in input.bytes() {
            santa = self.make_move(santa, b);

            self.visit_house(&mut visited_houses, santa);
        }

        visited_houses.len().to_string()
    }

    fn part_two(&self, input: &str) -> String {
        let mut visited_houses: Houses = HashMap::new();
        let mut santa = Point::new(0, 0);
        visited_houses.insert(santa, 1);

        let mut robo_santa = Point::new(0, 0);
        visited_houses.insert(robo_santa, 1);

        for (i, b) in input.bytes().enumerate() {
            let visited = if i % 2 == 0 {
                santa = self.make_move(santa, b);

                santa
            } else {
                robo_santa = self.make_move(robo_santa, b);

                robo_santa
            };

            self.visit_house(&mut visited_houses, visited);
        }

        visited_houses.len().to_string()
    }
}

impl Day03 {
    fn make_move(&self, point: Point, b: u8) -> Point {
        match b {
            b'>' => point.east(),
            b'<' => point.west(),
            b'^' => point.north(),
            b'v' => point.south(),
            _ => unreachable!(),
        }
    }

    fn visit_house(&self, houses: &mut Houses, house: Point) {
        houses.entry(house).and_modify(|e| *e += 1).or_insert(1);
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

    #[test]
    fn part_two_example_test() {
        assert_eq!("3", Day03.part_two("^v"));
        assert_eq!("3", Day03.part_two("^>v<"));
        assert_eq!("11", Day03.part_two("^v^v^v^v^v"));
    }
}
