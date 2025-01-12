use crate::solutions::year2024::day25::LockAndKey::{Key, Lock};
use crate::solutions::Solution;
use crate::utils::grid::Grid;
use std::str::FromStr;

const MAX_HEIGHT: u8 = 6;

pub struct Day25;

impl Solution for Day25 {
    fn part_one(&self, input: &str) -> String {
        let items: Vec<LockAndKey> = input
            .split_terminator("\n\n")
            .map(|item| item.parse().unwrap())
            .collect();

        let locks = Self::filter_items(&items, |item| matches!(item, Lock(_)));
        let keys = Self::filter_items(&items, |item| matches!(item, Key(_)));

        let mut overlap_count = 0;
        for lock in &locks {
            for key in &keys {
                if key.overlaps(lock) {
                    overlap_count += 1;
                }
            }
        }

        overlap_count.to_string()
    }

    fn part_two(&self, _input: &str) -> String {
        String::from("0")
    }
}

impl Day25 {
    fn filter_items(items: &[LockAndKey], item_type: fn(&LockAndKey) -> bool) -> Vec<&LockAndKey> {
        items.iter().filter(|item| item_type(item)).collect()
    }
}

#[derive(Debug)]
struct Pins(Vec<u8>);

#[derive(Debug)]
enum LockAndKey {
    Lock(Pins),
    Key(Pins),
}

impl LockAndKey {
    fn overlaps(&self, other: &Self) -> bool {
        if !(matches!(self, Key(_)) && matches!(other, Lock(_))
            || matches!(self, Lock(_)) && matches!(other, Key(_)))
        {
            unreachable!()
        }

        let self_pins = self.pins();
        let other_pins = other.pins();

        for i in 0..self_pins.0.len() {
            if self_pins.0[i] + other_pins.0[i] >= MAX_HEIGHT {
                return false;
            }
        }

        true
    }

    fn pins(&self) -> &Pins {
        match self {
            Lock(pins) => pins,
            Key(pins) => pins,
        }
    }
}

impl FromStr for LockAndKey {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let grid: Grid<char> = Grid::from(s);
        let pins: Vec<u8> = grid
            .columns()
            .values()
            .map(|c| (c.iter().filter(|(_, char)| **char == &'#').count() - 1) as u8)
            .collect();

        Ok(match grid.top_left_corner_element().unwrap() {
            '#' => Lock(Pins(pins)),
            '.' => Key(Pins(pins)),
            _ => unreachable!(),
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::solutions::year2024::day25::Day25;
    use crate::solutions::Solution;

    const EXAMPLE: &str = r#"#####
.####
.####
.####
.#.#.
.#...
.....

#####
##.##
.#.##
...##
...#.
...#.
.....

.....
#....
#....
#...#
#.#.#
#.###
#####

.....
.....
#.#..
###..
###.#
###.#
#####

.....
.....
.....
#....
#.#..
#.#.#
#####"#;

    #[test]
    fn part_one_example() {
        assert_eq!("3", Day25.part_one(EXAMPLE));
    }
}
