use crate::solutions::year2016::day11::Item::{Generator, Microchip};
use crate::solutions::Solution;
use itertools::Itertools;
use regex::Regex;
use std::collections::{HashSet, VecDeque};
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
        let mut visited: HashSet<State> = HashSet::new();

        queue.push_back((state.clone(), 0));
        visited.insert(state.sorted());

        while let Some((state, moves)) = queue.pop_front() {
            if state.is_finished() {
                return moves.to_string();
            }

            for next_state in state.possible_next_states() {
                let canonical = next_state.sorted();
                if visited.insert(canonical) {
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

    fn sorted(&self) -> Self {
        let floors = self
            .floors
            .iter()
            .map(|floor| {
                let mut items = floor.items.clone();
                items.sort_unstable();

                Floor::new(items)
            })
            .collect_vec();

        Self {
            floors,
            elevator: self.elevator,
        }
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
}
