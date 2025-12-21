use crate::solutions::Solution;
use itertools::Itertools;
use std::collections::HashMap;
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

    fn part_two(&self, input: &str) -> String {
        let reindeers = self.parse(input);
        let points = self.points(&reindeers, TIME);

        points
            .iter()
            .max_by_key(|(_, distance)| *distance)
            .unwrap()
            .1
            .to_string()
    }
}

impl Day14 {
    fn parse(&self, input: &str) -> Vec<Reindeer> {
        input.lines().map(|line| line.parse().unwrap()).collect()
    }

    fn points<'a>(&self, reindeers: &'a [Reindeer], time: u64) -> HashMap<&'a Reindeer, u64> {
        let mut points: HashMap<&Reindeer, u64> = HashMap::new();

        for reindeer in reindeers {
            points.insert(reindeer, 0);
        }

        for second in 1..=time {
            let leading = reindeers
                .iter()
                .map(|reindeer| (reindeer, reindeer.distance(second)))
                .max_set_by_key(|(_, distance)| *distance);

            for (lead_reindeer, _) in leading {
                *points.entry(lead_reindeer).or_default() += 1;
            }
        }

        points
    }
}

#[derive(Debug, Eq, Hash, PartialEq, Copy, Clone)]
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

    #[test]
    fn points() {
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

        let reindeers = &[comet, dancer];

        const ONE_SECOND: u64 = 1;

        let points = Day14.points(reindeers, ONE_SECOND);
        assert_eq!(*points.get(&dancer).unwrap(), 1);
        assert_eq!(*points.get(&comet).unwrap(), 0);

        const THOUSANDS_SECOND: u64 = 1000;

        let points = Day14.points(reindeers, THOUSANDS_SECOND);
        assert_eq!(*points.get(&dancer).unwrap(), 689);
        assert_eq!(*points.get(&comet).unwrap(), 312);
    }
}
