use crate::solutions::Solution;
use itertools::Itertools;
use std::str::FromStr;

pub struct Day02;

impl Solution for Day02 {
    fn part_one(&self, input: &str) -> String {
        self.parse(input)
            .map(|cuboid| cuboid.surface_area() + cuboid.smallest_side_area())
            .sum::<u64>()
            .to_string()
    }

    fn part_two(&self, input: &str) -> String {
        self.parse(input)
            .map(|cuboid| {
                let around_perimeter = cuboid
                    .perimeters()
                    .iter()
                    .sorted()
                    .take(2)
                    .map(|d| 2 * d)
                    .sum::<u64>();
                let bow = cuboid.perimeters().iter().product::<u64>();

                around_perimeter + bow
            })
            .sum::<u64>()
            .to_string()
    }
}

impl Day02 {
    fn parse<'a>(&self, input: &'a str) -> impl Iterator<Item = RectangularCuboid> + 'a {
        input.lines().map(|line| line.parse().unwrap())
    }
}

struct RectangularCuboid {
    width: u64,
    length: u64,
    height: u64,
}

impl RectangularCuboid {
    fn surface_area(&self) -> u64 {
        2 * self.area_width_length() + 2 * self.area_width_height() + 2 * self.area_length_height()
    }

    fn smallest_side_area(&self) -> u64 {
        *[
            self.area_width_length(),
            self.area_width_height(),
            self.area_length_height(),
        ]
        .iter()
        .min()
        .unwrap()
    }

    fn area_width_length(&self) -> u64 {
        self.width * self.length
    }

    fn area_width_height(&self) -> u64 {
        self.width * self.height
    }

    fn area_length_height(&self) -> u64 {
        self.length * self.height
    }

    fn perimeters(&self) -> [u64; 3] {
        [self.width, self.length, self.height]
    }
}

impl FromStr for RectangularCuboid {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (width, length, height) = s
            .split_terminator('x')
            .map(|s| s.parse::<u64>().unwrap())
            .collect_tuple()
            .unwrap();

        Ok(Self {
            width,
            length,
            height,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_one_example_test() {
        assert_eq!("58", Day02.part_one("2x3x4"));
        assert_eq!("43", Day02.part_one("1x1x10"));
    }

    #[test]
    fn part_two_example_test() {
        assert_eq!("34", Day02.part_two("2x3x4"));
        assert_eq!("14", Day02.part_two("1x1x10"));
    }
}
