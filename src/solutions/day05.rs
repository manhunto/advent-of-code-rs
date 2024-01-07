use crate::range::Range;
use crate::solutions::Solution;
use std::collections::HashMap;
use std::str;

pub struct Day05;

impl Solution for Day05 {
    fn part_one(&self, input: &str) -> String {
        let (seeds, maps) = parse_input_part_one(input);

        seeds
            .iter()
            .map(|seed| {
                let mut tmp: i64 = *seed;
                for map in &maps {
                    tmp = map.move_seed(tmp);
                }

                tmp
            })
            .min()
            .unwrap()
            .to_string()
    }

    fn part_two(&self, input: &str) -> String {
        let (seeds, maps) = parse_input_part_two(input);

        let mut seeds_all = seeds;
        for map in maps {
            let mut processed_seeds: Vec<Range> = vec![];

            let mut idx: i32 = 0;
            let mut count: i32 = seeds_all.len() as i32;

            let mut seeds_hash: HashMap<i32, Range> = HashMap::new();
            for s in 0..seeds_all.len() {
                seeds_hash.insert(s as i32, *seeds_all.get(s).unwrap());
            }

            while idx < count {
                let (left, moved) = map.move_seeds(*seeds_hash.get(&idx).unwrap());
                processed_seeds.push(moved);

                for l in left {
                    seeds_hash.insert(seeds_hash.len() as i32, l);
                    count += 1;
                }

                idx += 1;
            }

            seeds_all = processed_seeds.clone();
        }

        seeds_all
            .iter()
            .map(|seed| seed.start())
            .min()
            .unwrap()
            .to_string()
    }
}

fn parse_input_part_one(input: &str) -> (Vec<i64>, Vec<Map>) {
    let mut seeds: Vec<i64> = vec![];
    let mut maps: HashMap<&str, Vec<MapRange>> = HashMap::new();
    let mut maps_ordering: Vec<&str> = vec![];

    for line in input.lines() {
        if line.starts_with("seeds") {
            seeds = line[7..line.len()]
                .split_ascii_whitespace()
                .map(|v| v.parse().unwrap())
                .collect();
        } else if line.ends_with(" map:") {
            maps_ordering.push(line);
        } else if !line.is_empty() {
            let numbers = line
                .split_ascii_whitespace()
                .map(|v| v.parse().unwrap())
                .collect::<Vec<i64>>();

            let mut numbers = numbers.iter();

            let map_range = MapRange::new(
                *numbers.next().unwrap(),
                *numbers.next().unwrap(),
                *numbers.next().unwrap(),
            );

            maps.entry(maps_ordering.last().unwrap())
                .and_modify(|map| map.push(map_range.clone()))
                .or_insert(vec![map_range.clone()]);
        }
    }

    let mut maps_all: Vec<Map> = vec![];

    for map_name in maps_ordering {
        maps_all.push(Map::new(maps.get(map_name).unwrap().to_vec()))
    }

    (seeds, maps_all)
}

fn parse_input_part_two(input: &str) -> (Vec<Range>, Vec<Map>) {
    let (seeds, maps) = parse_input_part_one(input);

    let seeds_ranges: Vec<Range> = seeds
        .chunks(2)
        .map(|c| Range::with_length(*c.first().unwrap(), *c.get(1).unwrap()).unwrap())
        .collect();

    (seeds_ranges, maps)
}

#[derive(Debug, PartialEq)]
struct Map {
    maps: Vec<MapRange>,
}

impl Map {
    fn new(maps: Vec<MapRange>) -> Self {
        Map { maps }
    }
    fn move_seed(&self, source: i64) -> i64 {
        for map in &self.maps {
            if map.contains(source) {
                return map.move_seed(source).unwrap();
            }
        }

        source
    }

    fn move_seeds(&self, source: Range) -> (Vec<Range>, Range) {
        for map in &self.maps {
            if map.collide(source) {
                let left = source.diff(&map.range);
                let intersect = map.range.intersect(&source).unwrap();

                let diff = map.destination - map.range.start();
                let new_start = intersect.start() + diff;

                let moved = intersect.move_start_at(new_start).unwrap();

                return (left, moved);
            }
        }

        (vec![], source)
    }
}

#[derive(Debug, Clone, PartialEq)]
struct MapRange {
    range: Range,
    destination: i64,
    length: i64,
}

impl MapRange {
    fn new(destination: i64, source: i64, length: i64) -> Self {
        Self {
            range: Range::with_length(source, length).unwrap(),
            destination,
            length,
        }
    }
    fn contains(&self, source: i64) -> bool {
        self.range.is_in_range(source)
    }

    fn collide(&self, source: Range) -> bool {
        self.range.collide(&source)
    }

    fn move_seed(&self, source: i64) -> Option<i64> {
        if self.range.is_in_range(source) {
            let diff = source - self.range.start();
            return Some(self.destination + diff);
        }

        None
    }
}

#[cfg(test)]
mod tests {
    use crate::file_system::read_example;
    use crate::range::Range;
    use crate::solutions::day05::{parse_input_part_one, Day05, Map, MapRange};
    use crate::solutions::Solution;
    use std::vec;

    #[test]
    fn part_one_example_test() {
        let input = read_example("05");

        assert_eq!("35", Day05.part_one(input.as_str()));
    }

    #[test]
    fn part_two_example_test() {
        let input = read_example("05");

        assert_eq!("46", Day05.part_two(input.as_str()));
    }

    #[test]
    fn parse_input_part_one_test() {
        let input = read_example("05");

        let seeds: Vec<i64> = vec![79, 14, 55, 13];

        assert_eq!(
            (
                seeds,
                vec![
                    Map::new(vec![MapRange::new(50, 98, 2), MapRange::new(52, 50, 48),]),
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
                    Map::new(vec![MapRange::new(88, 18, 7), MapRange::new(18, 25, 70),]),
                    Map::new(vec![
                        MapRange::new(45, 77, 23),
                        MapRange::new(81, 45, 19),
                        MapRange::new(68, 64, 13),
                    ]),
                    Map::new(vec![MapRange::new(0, 69, 1), MapRange::new(1, 0, 69),]),
                    Map::new(vec![MapRange::new(60, 56, 37), MapRange::new(56, 93, 4),]),
                ]
            ),
            parse_input_part_one(&input)
        );
    }

    #[test]
    fn map_range_contains() {
        let range = MapRange::new(0, 5, 3);

        assert!(!range.contains(4));
        assert!(range.contains(5));
        assert!(range.contains(6));
        assert!(range.contains(7));
        assert!(!range.contains(8));
    }

    #[test]
    fn map_range_move_seed() {
        assert_eq!(81, MapRange::new(52, 50, 48).move_seed(79).unwrap());
        assert_eq!(57, MapRange::new(52, 50, 48).move_seed(55).unwrap());
    }

    #[test]
    fn map_move_seed() {
        let map = Map::new(vec![MapRange::new(50, 98, 2), MapRange::new(52, 50, 48)]);

        assert_eq!(81, map.move_seed(79));
        assert_eq!(14, map.move_seed(14));
        assert_eq!(57, map.move_seed(55));
        assert_eq!(13, map.move_seed(13));
    }

    #[test]
    fn map_move_seeds_handle_split_and_move() {
        let map = Map::new(vec![
            MapRange::new(45, 77, 23),
            MapRange::new(81, 45, 19),
            MapRange::new(68, 64, 13),
        ]);

        let seed = Range::new(74, 87).unwrap();

        let (left, moved) = map.move_seeds(seed);

        assert_eq!(vec![Range::new(74, 76).unwrap()], left);
        assert_eq!(Range::new(45, 55).unwrap(), moved);
    }
}
