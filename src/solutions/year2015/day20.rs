use crate::solutions::Solution;
use crate::utils::math::Math;

pub struct Day20;

impl Solution for Day20 {
    fn part_one(&self, input: &str) -> String {
        let presents_count: usize = input.trim().parse().unwrap();

        let mut sum: usize = 0;
        for house in 1usize.. {
            sum += self.presents_in_house(house);

            if sum >= presents_count {
                return house.to_string();
            }
        }

        unreachable!();
    }

    fn part_two(&self, _input: &str) -> String {
        String::from("0")
    }
}

impl Day20 {
    fn presents_in_house(&self, house: usize) -> usize {
        house.divisors().map(|n| n * 10).sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn presents_in_house() {
        assert_eq!(Day20.presents_in_house(1), 10);
        assert_eq!(Day20.presents_in_house(2), 30);
        assert_eq!(Day20.presents_in_house(3), 40);
        assert_eq!(Day20.presents_in_house(4), 70);
        assert_eq!(Day20.presents_in_house(5), 60);
        assert_eq!(Day20.presents_in_house(6), 120);
        assert_eq!(Day20.presents_in_house(7), 80);
        assert_eq!(Day20.presents_in_house(8), 150);
        assert_eq!(Day20.presents_in_house(9), 130);
    }
}
