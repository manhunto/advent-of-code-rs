use crate::solutions::Solution;
use crate::utils::math::Math;

pub struct Day20;

impl Solution for Day20 {
    fn part_one(&self, input: &str) -> String {
        let presents_count: usize = input.trim().parse().unwrap();

        (1usize..)
            .find(|house| self.presents_in_house_part_one(*house) >= presents_count)
            .unwrap()
            .to_string()
    }

    fn part_two(&self, input: &str) -> String {
        let presents_count: usize = input.trim().parse().unwrap();

        (1usize..)
            .find(|house| self.presents_in_house_part_two(*house) >= presents_count)
            .unwrap()
            .to_string()
    }
}

impl Day20 {
    fn presents_in_house_part_one(&self, house: usize) -> usize {
        house.divisors().map(|elf| elf * 10).sum()
    }

    fn presents_in_house_part_two(&self, house: usize) -> usize {
        house
            .divisors()
            .filter(|elf| house / elf <= 50)
            .map(|elf| elf * 11)
            .sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn presents_in_house() {
        assert_eq!(Day20.presents_in_house_part_one(1), 10);
        assert_eq!(Day20.presents_in_house_part_one(2), 30);
        assert_eq!(Day20.presents_in_house_part_one(3), 40);
        assert_eq!(Day20.presents_in_house_part_one(4), 70);
        assert_eq!(Day20.presents_in_house_part_one(5), 60);
        assert_eq!(Day20.presents_in_house_part_one(6), 120);
        assert_eq!(Day20.presents_in_house_part_one(7), 80);
        assert_eq!(Day20.presents_in_house_part_one(8), 150);
        assert_eq!(Day20.presents_in_house_part_one(9), 130);
    }
}
