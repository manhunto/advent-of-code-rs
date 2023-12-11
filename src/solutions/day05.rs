use std::collections::HashMap;
use crate::solutions::Solution;
use std::str;
use regex::{Captures, Regex};

pub struct Day05;

impl Solution for Day05 {
    fn part_one(&self, input: &str) -> String {
        String::from("0")
    }

    fn part_two(&self, input: &str) -> String {
        String::from("0")
    }
}

fn parse_input(input: &str) -> (Vec<i32>, Vec<Map>) {
    let mut seeds: Vec<i32> = vec![];
    let mut maps: HashMap<&str, Vec<MapRange>> = HashMap::new();
    let mut maps_ordering: Vec<&str> = vec![];

    let get_number = |captures: &Captures, key: usize| {
        captures.get(key).unwrap().as_str().parse().unwrap()
    };

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
                destination: get_number(&captures, 1),
                source: get_number(&captures, 2),
                length: get_number(&captures, 3),
            };

            maps
                .entry(maps_ordering.last().unwrap())
                .and_modify(|map| { map.push(map_range.clone()) })
                .or_insert(vec![map_range.clone()]);
        }
    }

    let mut maps_all: Vec<Map> = vec![];

    for map_name in maps_ordering {
        maps_all.push(Map::new(maps.get(map_name).unwrap().to_vec()))
    }

    (seeds, maps_all)
}

#[derive(Debug, PartialEq)]
struct Map {
    maps: Vec<MapRange>,
}

impl Map {
    fn new(maps: Vec<MapRange>) -> Self {
        Map {
            maps
        }
    }
    fn move_seed(&self, source: i32) -> i32 {
        for map in &self.maps {
            if map.contains(source) {
                return map.move_seed(source).unwrap();
            }
        }

        return source;
    }
}

#[derive(Debug, Clone, PartialEq)]
struct MapRange {
    destination: i32,
    source: i32,
    length: i32,
}

impl MapRange {
    fn new(destination: i32, source: i32, length: i32) -> Self {
        Self {
            destination,
            source,
            length,
        }
    }
    fn contains(&self, source: i32) -> bool {
        let range = self.source..self.source + self.length;
        range.contains(&source)
    }

    fn move_seed(&self, source: i32) -> Option<i32> {
        if self.contains(source) {
            let diff = source - self.source;
            return Some(self.destination + diff);
        }

        return None;
    }
}

#[cfg(test)]
mod tests {
    use std::vec;
    use crate::file_system::read_example;
    use crate::solutions::day05::{Day05, Map, MapRange, parse_input};
    use crate::solutions::Solution;

    #[test]
    fn part_one_example_test() {
        let input = read_example("05");

        assert_eq!("35", Day05.part_one(&input.as_str()));
    }

    #[test]
    fn parse_input_test() {
        let input = read_example("05");

        let seeds: Vec<i32> = vec![79, 14, 55, 13];

        assert_eq!(
            (seeds,
             vec![
                 Map::new(vec![
                     MapRange::new(50, 98, 2),
                     MapRange::new(52, 50, 48),
                 ]),
                 Map::new(vec![
                     MapRange::new(0, 15, 37),
                     MapRange::new(37, 52, 2),
                     MapRange::new(39, 0, 15),
                 ]),
                 Map::new(vec![
                     MapRange::new(49, 53, 8),
                     MapRange::new(0, 11, 42),
                     MapRange::new(42, 0, 7),
                     MapRange::new(57, 7, 4),
                 ]),
                 Map::new(vec![
                     MapRange::new(88, 18, 7),
                     MapRange::new(18, 25, 70),
                 ]),
                 Map::new(vec![
                     MapRange::new(45, 77, 23),
                     MapRange::new(81, 45, 19),
                     MapRange::new(68, 64, 13),
                 ]),
                 Map::new(vec![
                     MapRange::new(0, 69, 1),
                     MapRange::new(1, 0, 69),
                 ]),
                 Map::new(vec![
                     MapRange::new(60, 56, 37),
                     MapRange::new(56, 93, 4),
                 ]),
             ]
            ), parse_input(&input));
    }

    #[test]
    fn map_range_contains() {
        assert!(!MapRange::new(0, 5, 3).contains(4));
        assert!(MapRange::new(0, 5, 3).contains(5));
        assert!(MapRange::new(0, 5, 3).contains(6));
        assert!(MapRange::new(0, 5, 3).contains(7));
        assert!(!MapRange::new(0, 5, 3).contains(8));
    }

    #[test]
    fn map_range_move_seed() {
        assert_eq!(81, MapRange::new(52, 50, 48).move_seed(79).unwrap());
        assert_eq!(57, MapRange::new(52, 50, 48).move_seed(55).unwrap());
    }

    #[test]
    fn map_move_seed() {
        let map = Map::new(vec![
            MapRange::new(50, 98, 2),
            MapRange::new(52, 50, 48),
        ]);

        assert_eq!(81, map.move_seed(79));
        assert_eq!(14, map.move_seed(14));
        assert_eq!(57, map.move_seed(55));
        assert_eq!(13, map.move_seed(13));
    }
}
