use crate::solutions::Solution;
use crate::utils::point::Point;
use std::collections::HashMap;

type Houses = HashMap<Point, u64>;

pub struct Day03;

impl Solution for Day03 {
    fn part_one(&self, input: &str) -> String {
        self.solve(input, 1).to_string()
    }

    fn part_two(&self, input: &str) -> String {
        self.solve(input, 2).to_string()
    }
}

impl Day03 {
    fn solve(&self, input: &str, santas: usize) -> usize {
        let mut visited_houses: Houses = HashMap::new();
        let mut santas = vec![Point::new(0, 0); santas];

        for santa in &santas {
            visited_houses.insert(*santa, 1);
        }

        for (i, b) in input.bytes().enumerate() {
            let index = i % santas.len();

            santas[index] = self.make_move(santas[index], b);

            self.visit_house(&mut visited_houses, santas[index]);
        }

        visited_houses.len()
    }

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
        *houses.entry(house).or_default() += 1
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
