use crate::solutions::Solution;
use std::cmp::Ordering;
use std::fmt::{Display, Formatter};
use std::str::FromStr;

pub struct Day09;

impl Solution for Day09 {
    fn part_one(&self, input: &str) -> String {
        let mut disk_map = DiskMap::from_str(input).unwrap();
        let mut last_seen_digit_index = usize::MAX;

        loop {
            let last_digit_index = disk_map
                .blocks
                .iter()
                .take(last_seen_digit_index)
                .rposition(|v| v.is_some())
                .unwrap();

            let first_empty_index = disk_map.blocks.iter().position(|v| v.is_none()).unwrap();

            last_seen_digit_index = last_digit_index;

            if first_empty_index > last_digit_index {
                break;
            }

            disk_map.blocks[first_empty_index] = disk_map.blocks[last_digit_index].take();
        }

        disk_map.checksum().to_string()
    }

    fn part_two(&self, input: &str) -> String {
        let mut map = BlockDiskMap::from_str(input).unwrap();
        let mut last_checked_index = usize::MAX;

        while let Some(filled_unwrapped) = map.last_filled_until_index(last_checked_index) {
            last_checked_index = filled_unwrapped.0;

            if let Some(first_empty_spot) =
                map.first_empty_until_index(last_checked_index, &filled_unwrapped.1)
            {
                map.place_filled_in_empty(filled_unwrapped, first_empty_spot);
            }
        }

        DiskMap::from(map).checksum().to_string()
    }
}

struct DiskMap {
    blocks: Vec<Option<usize>>,
}

impl DiskMap {
    fn checksum(&self) -> usize {
        self.blocks
            .iter()
            .enumerate()
            .map(|(i, id)| id.map_or(0, |id| i * id))
            .sum()
    }
}

impl FromStr for DiskMap {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let blocks: Vec<Option<usize>> = s
            .trim()
            .chars()
            .enumerate()
            .flat_map(|(i, c)| {
                let times: usize = c.to_digit(10).expect("cannot parse char to usize") as usize;
                let value = if i % 2 == 0 { Some(i / 2) } else { None };

                vec![value; times]
            })
            .collect();

        Ok(Self { blocks })
    }
}

impl Display for DiskMap {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let value: String = self
            .blocks
            .iter()
            .map(|v| match v {
                None => '.',
                Some(v) => char::from_digit((v % 10) as u32, 10).unwrap(),
            })
            .collect();

        write!(f, "{}", value)
    }
}

impl From<BlockDiskMap> for DiskMap {
    fn from(value: BlockDiskMap) -> Self {
        let blocks = value
            .blocks
            .into_iter()
            .flat_map(|v| match v {
                Block::Empty { size } => vec![None; size],
                Block::Filled { size, id } => vec![Some(id); size],
            })
            .collect();

        Self { blocks }
    }
}

#[derive(Debug, PartialEq, Clone)]
enum Block {
    Empty { size: usize },
    Filled { size: usize, id: usize },
}

impl Block {
    fn empty(size: usize) -> Self {
        Self::Empty { size }
    }

    fn filled(size: usize, id: usize) -> Self {
        Self::Filled { size, id }
    }

    fn size(&self) -> usize {
        match self {
            Block::Empty { size } => size.to_owned(),
            Block::Filled { size, .. } => size.to_owned(),
        }
    }

    fn split(&self, other: &Self) -> Vec<Self> {
        match (self, other) {
            (
                Block::Empty { size: empty_size },
                Block::Filled {
                    size: filled_size,
                    id,
                },
            ) => {
                assert!(
                    *filled_size <= *empty_size,
                    "filled size cannot be greater than empty"
                );

                match filled_size.cmp(empty_size) {
                    Ordering::Equal => vec![Block::filled(*filled_size, *id)],
                    Ordering::Less => vec![
                        Block::filled(*filled_size, *id),
                        Block::empty(*empty_size - filled_size),
                    ],
                    Ordering::Greater => unreachable!(),
                }
            }
            _ => panic!("illegal block"),
        }
    }
}

#[derive(Debug, PartialEq)]
struct BlockDiskMap {
    blocks: Vec<Block>,
}

impl FromStr for BlockDiskMap {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let blocks: Vec<Block> = s
            .trim()
            .chars()
            .enumerate()
            .map(|(i, c)| {
                let size: usize = c.to_digit(10).expect("cannot parse char to usize") as usize;

                if i % 2 == 0 {
                    Block::filled(size, i / 2)
                } else {
                    Block::empty(size)
                }
            })
            .collect();

        Ok(Self { blocks })
    }
}

impl BlockDiskMap {
    fn last_filled_until_index(&self, max_index: usize) -> Option<(usize, Block)> {
        self.blocks
            .iter()
            .take(max_index)
            .enumerate()
            .rfind(|(_, block)| matches!(block, Block::Filled { .. }))
            .map(|(i, block)| (i, block.clone()))
    }

    fn first_empty_until_index(&self, max_index: usize, other: &Block) -> Option<(usize, Block)> {
        self.blocks
            .iter()
            .take(max_index)
            .enumerate()
            .find(|(_, block)| matches!(block, Block::Empty { .. }) && block.size() >= other.size())
            .map(|(i, block)| (i, block.clone()))
    }

    fn place_filled_in_empty(&mut self, filled: (usize, Block), empty: (usize, Block)) {
        let (empty_index, empty_block) = empty;
        let (filled_index, filled_block) = filled;

        if !matches!(empty_block, Block::Empty { .. }) {
            panic!("should be empty")
        }

        if !matches!(filled_block, Block::Filled { .. }) {
            panic!("should be filled")
        }

        let split = empty_block.split(&filled_block);

        self.blocks[filled_index] = Block::empty(filled_block.size());
        self.blocks.remove(empty_index);
        self.blocks.splice(empty_index..empty_index, split);
    }
}

#[cfg(test)]
mod tests {
    use crate::solutions::year2024::day09::{Block, BlockDiskMap, Day09, DiskMap};
    use crate::solutions::Solution;
    use std::str::FromStr;

    const EXAMPLE: &str = "2333133121414131402";

    #[test]
    fn part_one_example_test() {
        assert_eq!("1928", Day09.part_one(EXAMPLE));
    }

    #[test]
    fn part_two_example_test() {
        assert_eq!("2858", Day09.part_two(EXAMPLE));
    }

    #[test]
    fn disk_map_parse_test() {
        let result = DiskMap::from_str("12345").unwrap();
        assert_eq!("0..111....22222", result.to_string());

        let result = DiskMap::from_str(EXAMPLE).unwrap();
        assert_eq!(
            "00...111...2...333.44.5555.6666.777.888899",
            result.to_string()
        );
    }

    #[test]
    fn block_disk_map_parse_test() {
        let sut = BlockDiskMap::from_str("12345").unwrap();
        let expected = BlockDiskMap {
            blocks: vec![
                Block::filled(1, 0),
                Block::empty(2),
                Block::filled(3, 1),
                Block::empty(4),
                Block::filled(5, 2),
            ],
        };

        assert_eq!(expected, sut);
    }
}
