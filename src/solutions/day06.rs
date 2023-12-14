use std::fmt::format;
use crate::solutions::Solution;

pub struct Day06;

impl Solution for Day06 {
    fn part_one(&self, input: &str) -> String {
        let races = parse_input_part_one(input);

        races
            .iter()
            .map(|race| {
                let mut win_count = 0;
                for hold_sec in 0..=race.time {
                    let reaming_time = race.time - hold_sec;
                    let distance = hold_sec * reaming_time;

                    if distance > race.distance_to_beat {
                        win_count += 1;
                    }
                }

                win_count
            })
            .product::<i32>()
            .to_string()
    }

    fn part_two(&self, input: &str) -> String {
        String::from("0")
    }
}

fn parse_input_part_one(input: &str) -> Vec<RaceInfo> {
    let (times, distances) = pre_parse(input);

    times
        .iter()
        .enumerate()
        .map(|(i, t)| {
            RaceInfo::new(
                *t,
                *distances.get(i).unwrap()
            )
        }).collect()
}

fn parse_input_part_two(input: &str) -> RaceInfo {
    let (times, distances) = pre_parse(input);

    let get_number = |vec: Vec<i32>| {
        vec
            .iter()
            .fold(String::from(""), |acc, elem| format!("{}{}", acc, elem))
            .parse()
            .unwrap()
    };

    RaceInfo::new(
        get_number(times),
        get_number(distances)
    )
}

fn pre_parse(input: &str) -> (Vec<i32>, Vec<i32>) {
    let mut lines = input.lines();

    let get_numbers_from_line = |line: Option<&str>| -> Vec<i32> {
        line
            .unwrap()
            .split_whitespace()
            .filter_map(|part| part.parse::<i32>().ok())
            .collect()
    };

    let times: Vec<i32> = get_numbers_from_line(lines.next());
    let distances: Vec<i32> = get_numbers_from_line(lines.next());

    (times, distances)
}

#[derive(PartialEq,Debug)]
struct RaceInfo {
    time: i32,
    distance_to_beat: i32,
}

impl RaceInfo {
    fn new(time: i32, distance_to_beat: i32) -> Self {
        Self { time, distance_to_beat }
    }
}

#[cfg(test)]
mod tests {
    use crate::file_system::read_example;
    use crate::solutions::day06::{Day06, parse_input_part_one, parse_input_part_two, RaceInfo};
    use crate::solutions::Solution;

    #[test]
    fn part_one_example_test() {
        let input = read_example("06");

        assert_eq!("288", Day06.part_one(&input.as_str()));
    }

    #[test]
    fn parse_input_part_one_test() {
        let input = read_example("06");

        assert_eq!(vec![
            RaceInfo::new(7, 9),
            RaceInfo::new(15, 40),
            RaceInfo::new(30, 200),
        ], parse_input_part_one(&input))
    }

    #[test]
    fn parse_input_part_two_test() {
        let input = read_example("06");

        assert_eq!(RaceInfo::new(71530, 940200), parse_input_part_two(&input));
    }
}
