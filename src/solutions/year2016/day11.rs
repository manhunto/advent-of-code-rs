use crate::solutions::year2016::day11::Item::{Generator, Microchip};
use crate::solutions::Solution;
use itertools::Itertools;
use regex::Regex;
use std::collections::{HashMap, HashSet, VecDeque};
use std::fmt;
use std::fmt::{Debug, Formatter};
use std::hash::Hash;
use std::str::FromStr;

pub struct Day11;

impl Solution for Day11 {
    fn part_one(&self, input: &str) -> String {
        let floors = self.parse(input);

        self.solve(floors)
    }

    fn part_two(&self, input: &str) -> String {
        let mut floors = self.parse(input);
        let items_first_floor = [
            Microchip(b'e'),
            Microchip(b'd'),
            Generator(b'e'),
            Generator(b'd'),
        ];

        floors[0].items.extend_from_slice(&items_first_floor);

        self.solve(floors)
    }
}

impl Day11 {
    fn solve(&self, floors: Vec<Floor>) -> String {
        let state = State::new(floors).unwrap();

        let mut queue = VecDeque::new();
        let mut visited_hashes: HashSet<Vec<u8>> = HashSet::new();

        queue.push_back((state.clone(), 0));
        visited_hashes.insert(state.canonical_hash());

        while let Some((state, moves)) = queue.pop_front() {
            if state.is_finished() {
                return moves.to_string();
            }

            for next_state in state.possible_next_states() {
                if visited_hashes.insert(next_state.canonical_hash()) {
                    queue.push_back((next_state, moves + 1));
                }
            }
        }

        unreachable!()
    }

    fn parse(&self, input: &str) -> Vec<Floor> {
        let re = Regex::new(r"[a-z\-]+ (microchip|generator)").unwrap();

        input
            .lines()
            .map(|l| {
                let items: Vec<Item> = re
                    .captures_iter(l)
                    .filter_map(|cap| cap.get(0))
                    .map(|m| m.as_str().parse().unwrap())
                    .collect();

                Floor::new(items)
            })
            .collect()
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
struct State {
    elevator: u8,
    floors: Vec<Floor>,
}

impl State {
    fn new(floors: Vec<Floor>) -> Result<Self, &'static str> {
        Self::new_with_elevator(floors, 0)
    }

    fn new_with_elevator(floors: Vec<Floor>, elevator: u8) -> Result<Self, &'static str> {
        let new = Self { floors, elevator };

        if !new.is_valid() {
            return Err("Invalid floor state");
        }

        Ok(new)
    }

    fn is_valid(&self) -> bool {
        self.floors.iter().all(|floor| floor.is_valid())
    }

    fn is_finished(&self) -> bool {
        self.floors[0].items.is_empty()
            && self.floors[1].items.is_empty()
            && self.floors[2].items.is_empty()
    }

    fn possible_next_states(&self) -> Vec<Self> {
        let has_items_below = (0..self.elevator).any(|i| !self.floors[i as usize].items.is_empty());

        (1..=2)
            .flat_map(|k| self.items_on_current_floor().into_iter().combinations(k))
            .flat_map(|combo| {
                [-1, 1].iter().filter_map(move |floor_diff| {
                    if !has_items_below && *floor_diff == -1 {
                        return None;
                    }

                    self.move_items(&combo, *floor_diff).ok()
                })
            })
            .collect()
    }

    fn items_on_current_floor(&self) -> Vec<Item> {
        self.floors[self.elevator as usize].items.clone()
    }

    fn move_items(&self, combo: &[Item], elevator_diff: i32) -> Result<Self, &'static str> {
        let next_floor = self.elevator as i32 + elevator_diff;

        if !(0..=3).contains(&next_floor) {
            return Err("Invalid floor number");
        }

        let mut new_floors = self.floors.clone();
        new_floors[self.elevator as usize]
            .items
            .retain(|item| !combo.contains(item));
        new_floors[next_floor as usize]
            .items
            .extend_from_slice(combo);

        Self::new_with_elevator(new_floors, next_floor as u8)
    }

    /// Creates a canonical hash that identifies structurally equivalent states.
    ///
    /// Treats states as identical if they have the same pairing pattern, regardless of
    /// which specific elements are involved. This reduces the BFS search space.
    ///
    /// # Encoding
    ///
    /// - `3` = matched pair (generator + microchip of same element)
    /// - `1` = unpaired generator
    /// - `2` = unpaired microchip
    ///
    /// # Examples
    ///
    /// ```
    /// // [Generator('a')] → [0, '|', 1, '|']
    /// // [Microchip('a')] → [0, '|', 2, '|']
    /// // [Microchip('a'), Generator('a')] → [0, '|', 3, '|']
    ///
    /// // Two floors with identical structure produce the same encoding:
    /// // Floor 0: [Microchip('a'), Generator('a'), Generator('b')]
    /// // Floor 1: [Microchip('b'), Generator('b'), Generator('a')]
    /// // → [0, '|', 3, 1, '|', 3, 1, '|']
    /// //    Both floors: one pair + one unpaired generator
    /// ```
    fn canonical_hash(&self) -> Vec<u8> {
        let mut result = vec![self.elevator, b'|'];

        self.floors.iter().for_each(|floor| {
            let mut map: HashMap<u8, Vec<u8>> = HashMap::new();

            floor.items.iter().for_each(|item| {
                let i = match item {
                    Generator(_) => b'g',
                    Microchip(_) => b'm',
                };

                map.entry(item.value()).or_default().push(i);
            });

            let new = map
                .values()
                .map(|value| {
                    if value.len() == 2 {
                        return 3u8;
                    }

                    if value.contains(&b'g') {
                        return 1;
                    }

                    if value.contains(&b'm') {
                        return 2;
                    }

                    unreachable!()
                })
                .sorted()
                .rev()
                .collect_vec();

            result.extend(new);
            result.push(b'|');
        });

        result
    }
}

#[derive(Clone, Eq, PartialEq, Hash)]
struct Floor {
    items: Vec<Item>,
}

impl Floor {
    fn new(items: Vec<Item>) -> Self {
        Self { items }
    }

    /// microchip cannot be on the same floor with other generator, but only with own generator
    fn is_valid(&self) -> bool {
        if self.items.is_empty() {
            return true;
        }

        let has_other_generator = self
            .items
            .iter()
            .any(|item| matches!(item, Item::Generator(_)));

        if !has_other_generator {
            return true;
        }

        self.items
            .iter()
            .filter(|item| matches!(item, Item::Microchip(_)))
            .all(|microchip| self.items.contains(&microchip.opposite()))
    }
}

impl Debug for Floor {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self.items)
    }
}

#[derive(Copy, Clone, PartialEq, Eq, Ord, PartialOrd, Hash)]
enum Item {
    Generator(u8),
    Microchip(u8),
}

impl Item {
    fn opposite(&self) -> Self {
        match self {
            Generator(v) => Microchip(*v),
            Microchip(v) => Generator(*v),
        }
    }

    fn value(&self) -> u8 {
        match self {
            Generator(v) | Microchip(v) => *v,
        }
    }
}

impl FromStr for Item {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (name, item) = s.split_once(' ').unwrap();

        let n = name.as_bytes()[0];
        let t = item.as_bytes()[0];

        match t {
            b'm' => Ok(Item::Microchip(n)),
            b'g' => Ok(Item::Generator(n)),
            _ => Err(()),
        }
    }
}

impl Debug for Item {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let mut write = |c: char, v: u8| write!(f, "{}{}", (v as char).to_ascii_uppercase(), c);

        match self {
            Item::Generator(v) => write('G', *v),
            Item::Microchip(v) => write('M', *v),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::solutions::year2016::day11::Item::{Generator, Microchip};

    const EXAMPLE: &str = r#"The first floor contains a hydrogen-compatible microchip and a lithium-compatible microchip.
The second floor contains a hydrogen generator.
The third floor contains a lithium generator.
The fourth floor contains nothing relevant."#;

    #[test]
    fn part_one_example() {
        assert_eq!("11", Day11.part_one(EXAMPLE));
    }

    #[test]
    fn floor_is_valid() {
        assert!(Floor::new(vec![Microchip(0)]).is_valid());
        assert!(Floor::new(vec![Generator(0)]).is_valid());
        assert!(Floor::new(vec![Microchip(0), Generator(0)]).is_valid());
        assert!(!Floor::new(vec![Microchip(0), Generator(1)]).is_valid());
        assert!(Floor::new(vec![Microchip(0), Generator(1), Generator(0)]).is_valid());
        assert!(Floor::new(vec![Microchip(0), Microchip(1)]).is_valid());
        assert!(!Floor::new(vec![Microchip(0), Microchip(1), Generator(1)]).is_valid());
        assert!(
            Floor::new(vec![Microchip(0), Microchip(1), Generator(1), Generator(0)]).is_valid()
        );
    }

    #[test]
    fn state_hash_generator() {
        let state = State::new(vec![Floor::new(vec![Generator(b'a')])]).unwrap();

        assert_eq!([0, b'|', 1, b'|'], *state.canonical_hash())
    }

    #[test]
    fn state_hash_microchip() {
        let state = State::new(vec![Floor::new(vec![Microchip(b'a')])]).unwrap();

        assert_eq!([0, b'|', 2, b'|'], *state.canonical_hash())
    }

    #[test]
    fn state_hash_microchip_and_generator() {
        let state = State::new(vec![Floor::new(vec![Microchip(b'a'), Generator(b'a')])]).unwrap();

        assert_eq!([0, b'|', 3, b'|'], *state.canonical_hash())
    }

    #[test]
    fn state_hash_complex() {
        let floor1 = vec![Microchip(b'a'), Generator(b'a'), Generator(b'b')];
        let floor2 = vec![Generator(b'b'), Microchip(b'a'), Generator(b'a')];
        let state = State::new(vec![Floor::new(floor1), Floor::new(floor2)]).unwrap();

        assert_eq!([0, b'|', 3, 1, b'|', 3, 1, b'|'], *state.canonical_hash())
    }
}
