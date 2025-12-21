use crate::solutions::Solution;
use itertools::Itertools;
use std::num::ParseIntError;
use std::str::FromStr;

const TIME: u64 = 2503;

pub struct Day14;

impl Solution for Day14 {
    fn part_one(&self, input: &str) -> String {
        self.parse(input)
            .iter()
            .map(|reindeer| reindeer.distance(TIME))
            .max()
            .unwrap()
            .to_string()
    }

    fn part_two(&self, _input: &str) -> String {
        String::from("0")
    }
}

impl Day14 {
    fn parse(&self, input: &str) -> Vec<Reindeer> {
        input.lines().map(|line| line.parse().unwrap()).collect()
    }
}

#[derive(Debug)]
struct Reindeer {
    fly_speed: u64,
    fly_time: u64,
    rest_time: u64,
}

impl Reindeer {
    fn distance(&self, time: u64) -> u64 {
        let segment_time = self.fly_time + self.rest_time;
        let full_segments_in_time = time / segment_time;
        let base_distance = self.fly_speed * self.fly_time * full_segments_in_time;
        let missing_time = time - full_segments_in_time * segment_time;

        let run_time = missing_time.min(self.fly_time);

        base_distance + self.fly_speed * run_time
    }
}

impl FromStr for Reindeer {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts = s.split_ascii_whitespace().collect_vec();

        Ok(Self {
            fly_speed: parts[3].parse()?,
            fly_time: parts[6].parse()?,
            rest_time: parts[13].parse()?,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reindeer_distance() {
        let comet = Reindeer {
            fly_speed: 14,
            fly_time: 10,
            rest_time: 127,
        };

        let dancer = Reindeer {
            fly_speed: 16,
            fly_time: 11,
            rest_time: 162,
        };

        const ONE_SECOND: u64 = 1;
        assert_eq!(comet.distance(ONE_SECOND), 14);
        assert_eq!(dancer.distance(ONE_SECOND), 16);

        const TEN_SECONDS: u64 = 10;
        assert_eq!(comet.distance(TEN_SECONDS), 140);
        assert_eq!(dancer.distance(TEN_SECONDS), 160);

        const ELEVEN_SECONDS: u64 = 11;
        assert_eq!(comet.distance(ELEVEN_SECONDS), 140);
        assert_eq!(dancer.distance(ELEVEN_SECONDS), 176);

        const THOUSAND_SECONDS: u64 = 1_000;
        assert_eq!(comet.distance(THOUSAND_SECONDS), 1120);
        assert_eq!(dancer.distance(THOUSAND_SECONDS), 1056);
    }
}
