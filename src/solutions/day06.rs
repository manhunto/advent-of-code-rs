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
                    if race.is_winning_for_hold(hold_sec) {
                        win_count += 1;
                    }
                }

                win_count
            })
            .product::<i32>()
            .to_string()
    }

    fn part_two(&self, input: &str) -> String {
        let race = parse_input_part_two(input);

        let from = race.first_winning(0..=race.time);
        let to = race.first_winning((0..=race.time).rev());

        (to - from + 1).to_string()
    }
}

fn parse_input_part_one(input: &str) -> Vec<RaceInfo> {
    let (times, distances) = pre_parse(input);

    times
        .iter()
        .enumerate()
        .map(|(i, t)| RaceInfo::new(*t, *distances.get(i).unwrap()))
        .collect()
}

fn parse_input_part_two(input: &str) -> RaceInfo {
    let (times, distances) = pre_parse(input);

    let get_number = |vec: Vec<u64>| {
        vec.iter()
            .fold(String::from(""), |acc, elem| format!("{}{}", acc, elem))
            .parse()
            .unwrap()
    };

    RaceInfo::new(get_number(times), get_number(distances))
}

fn pre_parse(input: &str) -> (Vec<u64>, Vec<u64>) {
    let mut lines = input.lines();

    let get_numbers_from_line = |line: Option<&str>| -> Vec<u64> {
        line.unwrap()
            .split_whitespace()
            .filter_map(|part| part.parse::<u64>().ok())
            .collect()
    };

    let times: Vec<u64> = get_numbers_from_line(lines.next());
    let distances: Vec<u64> = get_numbers_from_line(lines.next());

    (times, distances)
}

#[derive(PartialEq, Debug)]
struct RaceInfo {
    time: u64,
    distance_to_beat: u64,
}

impl RaceInfo {
    fn new(time: u64, distance_to_beat: u64) -> Self {
        Self {
            time,
            distance_to_beat,
        }
    }

    fn is_winning_for_hold(&self, hold_sec: u64) -> bool {
        let remaining_time = self.time - hold_sec;
        let distance = hold_sec * remaining_time;

        distance > self.distance_to_beat
    }

    fn first_winning<I>(&self, range: I) -> u64
    where
        I: IntoIterator<Item = u64>,
    {
        for hold_sec in range {
            if self.is_winning_for_hold(hold_sec) {
                return hold_sec;
            }
        }

        panic!("This range is not winning");
    }
}

#[cfg(test)]
mod tests {
    use crate::file_system::read_example;
    use crate::solutions::day06::{parse_input_part_one, parse_input_part_two, Day06, RaceInfo};
    use crate::solutions::Solution;

    #[test]
    fn part_one_example_test() {
        let input = read_example("06");

        assert_eq!("288", Day06.part_one(input.as_str()));
    }

    #[test]
    fn part_two_example_test() {
        let input = read_example("06");

        assert_eq!("71503", Day06.part_two(input.as_str()));
    }

    #[test]
    fn parse_input_part_one_test() {
        let input = read_example("06");

        assert_eq!(
            vec![
                RaceInfo::new(7, 9),
                RaceInfo::new(15, 40),
                RaceInfo::new(30, 200),
            ],
            parse_input_part_one(&input)
        )
    }

    #[test]
    fn parse_input_part_two_test() {
        let input = read_example("06");

        assert_eq!(RaceInfo::new(71530, 940200), parse_input_part_two(&input));
    }
}
