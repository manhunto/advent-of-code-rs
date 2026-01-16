use crate::solutions::Solution;
use itertools::Itertools;
use regex::Regex;
use std::collections::{HashSet, VecDeque};
use std::fmt;
use std::fmt::{Debug, Formatter};
use std::hash::{Hash, Hasher};
use std::str::FromStr;

pub struct Day11;

impl Solution for Day11 {
    fn part_one(&self, input: &str) -> String {
        let floors = self.parse(input);
        let state = State::new(floors).unwrap();

        let mut queue = VecDeque::new();
        let mut visited: HashSet<State> = HashSet::new();

        for next_state in state.possible_next_states() {
            visited.insert(next_state.clone());
            queue.push_back(next_state);
        }

        while let Some(state) = queue.pop_front() {
            if state.is_finished() {
                return state.moves.to_string();
            }

            for next_state in state.possible_next_states() {
                if visited.insert(next_state.clone()) {
                    queue.push_back(next_state);
                }
            }
        }

        unreachable!()
    }

    fn part_two(&self, _input: &str) -> String {
        String::from("0")
    }
}

impl Day11 {
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

#[derive(Debug, Clone, Eq, PartialEq)]
struct State {
    elevator: u8,
    floors: Vec<Floor>,
    moves: usize,
}

impl State {
    fn new(floors: Vec<Floor>) -> Result<Self, &'static str> {
        let new = Self {
            floors,
            elevator: 0,
            moves: 0,
        };

        if !new.is_valid() {
            return Err("Invalid floor state");
        }

        Ok(new)
    }

    fn with_moved_items(&self, floors: Vec<Floor>, next_floor: u8) -> Result<Self, &'static str> {
        let new = Self {
            moves: self.moves + 1,
            floors,
            elevator: next_floor,
        };

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
            && !self.floors[3].items.is_empty()
    }

    fn possible_next_states(&self) -> Vec<Self> {
        (1..=2)
            .flat_map(|k| self.items_on_current_floor().into_iter().combinations(k))
            .flat_map(|combo| {
                [-1, 1]
                    .iter()
                    .filter_map(|floor_diff| self.move_items(&combo, *floor_diff).ok())
                    .collect_vec()
            })
            .collect()
    }

    fn items_on_current_floor(&self) -> Vec<Item> {
        self.floors[self.elevator as usize].items.clone()
    }

    fn move_items(&self, combo: &[Item], elevator_diff: i32) -> Result<Self, &'static str> {
        let next_floor = self.elevator as i32 - elevator_diff;

        if !(0..=3).contains(&next_floor) {
            return Err("Invalid floor number");
        }

        let new_floors: Vec<Floor> = self
            .floors
            .clone()
            .into_iter()
            .enumerate()
            .map(|(i, floor)| {
                if i == self.elevator as usize {
                    let mut current_floor_items = self.floors.get(i).unwrap().items.clone();
                    current_floor_items.retain(|item| !combo.contains(item));

                    return Floor::new(current_floor_items);
                }

                if i == next_floor as usize {
                    let mut new_floor_items = self.floors.get(i).unwrap().items.clone();
                    new_floor_items.extend(combo);

                    return Floor::new(new_floor_items);
                }

                floor
            })
            .collect();

        self.with_moved_items(new_floors, next_floor as u8)
    }
}

impl Hash for State {
    fn hash<H: Hasher>(&self, state: &mut H) {
        state.write_u8(self.elevator);
        state.write_u8(b'|');
        self.floors.hash(state);
    }
}

#[derive(Clone, Eq, PartialEq)]
struct Floor {
    items: Vec<Item>,
}

impl Floor {
    fn new(items: Vec<Item>) -> Self {
        Self { items }
    }

    /// microchip cannot be on the same floor with other generator, but only with own generator
    fn is_valid(&self) -> bool {
        let generators = self
            .items
            .iter()
            .filter(|item| matches!(item, Item::Generator(_)))
            .collect::<Vec<_>>();

        if generators.is_empty() {
            return true;
        }

        self.items
            .iter()
            .filter(|item| matches!(item, Item::Microchip(_)))
            .all(|microchip| generators.contains(&&microchip.opposite()))
    }
}

impl Debug for Floor {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self.items)
    }
}

impl Hash for Floor {
    fn hash<H: Hasher>(&self, state: &mut H) {
        let mut items = self.items.clone();
        items.sort();

        items.hash(state);
    }
}

#[derive(Copy, Clone, PartialEq, Eq, Ord, PartialOrd)]
enum Item {
    Generator(u8),
    Microchip(u8),
}

impl Item {
    fn opposite(&self) -> Self {
        match self {
            Item::Generator(v) => Item::Microchip(*v),
            Item::Microchip(v) => Item::Generator(*v),
        }
    }
}

impl Hash for Item {
    fn hash<H: Hasher>(&self, state: &mut H) {
        match self {
            Item::Generator(v) => {
                state.write_u8(b'G');
                state.write_u8(*v);
            }
            Item::Microchip(v) => {
                state.write_u8(b'M');
                state.write_u8(*v);
            }
        }
    }
}

impl FromStr for Item {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (name, item) = s.split_once(' ').unwrap();

        let n = name.as_bytes().first().unwrap();
        let t = item.as_bytes().first().unwrap();

        match t {
            b'm' => Ok(Item::Microchip(*n)),
            b'g' => Ok(Item::Generator(*n)),
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
