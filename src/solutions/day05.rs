use std::collections::HashMap;
use crate::solutions::Solution;
use std::str;
use regex::Regex;

pub struct Day05;

impl Solution for Day05 {
    fn part_one(&self, input: &str) -> String {
        String::from("0")
    }

    fn part_two(&self, input: &str) -> String {
        String::from("0")
    }
}

fn parse_input(input: &str) {
    let mut seeds: Vec<i32> = vec![];
    let mut maps: HashMap<&str, Vec<MapRange>> = HashMap::new();
    let mut maps_ordering: Vec<&str> = vec![];

    for line in input.lines() {
        if line.starts_with("seeds") {
            let re = Regex::new(r"(\d+)").unwrap();

            seeds = re
                .find_iter(line)
                .map(|m| m.as_str().parse().unwrap())
                .collect();
        } else if line.ends_with(" map:") {
            maps_ordering.push(line);
        } else if line.is_empty() == false {
            let re = Regex::new(r"(\d+)\s(\d+)\s(\d+)").unwrap();
            let captures = re.captures(line).unwrap();

            let map_range = MapRange {
                destination:captures.get(1).unwrap().as_str().parse().unwrap(),
                source: captures.get(2).unwrap().as_str().parse().unwrap(),
                length: captures.get(3).unwrap().as_str().parse().unwrap(),
            };

            maps
                .entry(maps_ordering.last().unwrap())
                .and_modify(|map| { map.push(map_range.clone()) })
                .or_insert(vec![map_range.clone()]);
        }
    }

    println!("{:?}", seeds);
    println!("{:?}", maps);
    println!("{:?}", maps_ordering);
}

#[derive(Debug, Clone)]
struct MapRange {
    destination: i32,
    source: i32,
    length: i32,
}

#[cfg(test)]
mod tests {
    use crate::file_system::read_example;
    use crate::solutions::day05::{Day05, parse_input};
    use crate::solutions::Solution;

    #[test]
    fn part_one_example_test() {
        let input = read_example("05");

        assert_eq!("35", Day05.part_one(&input.as_str()));
    }

    #[test]
    fn parse_input_test() {
        let input = read_example("05");

        parse_input(&input);
    }
}
