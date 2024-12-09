use crate::solutions::Solution;
use itertools::Itertools;
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
        let mut block_disk_map = BlockDiskMap::from_str(input).unwrap();
        let mut last_checked_index = usize::MAX;

        loop {
            let cloned = block_disk_map.blocks.clone();
            let last_filled_block = cloned
                .iter()
                .take(last_checked_index)
                .enumerate()
                .rfind(|(_, block)| matches!(block, Block::Filled { .. }));

            if last_filled_block.is_none() {
                break;
            }

            let filled_unwrapped = last_filled_block.unwrap();
            last_checked_index = filled_unwrapped.0;

            let cloned = block_disk_map.blocks.clone();
            let first_matching_spot =
                cloned
                    .iter()
                    .enumerate()
                    .take(filled_unwrapped.0)
                    .find(|(_, block)| match block {
                        Block::Empty { size } => size >= &filled_unwrapped.1.size(),
                        _ => false,
                    });

            if first_matching_spot.is_none() {
                continue;
            }

            let (empty_index, matching_block) = first_matching_spot.unwrap();
            let (filled_index, filled_block) = last_filled_block.unwrap();

            let split = matching_block.split(filled_block);

            block_disk_map.blocks[filled_index] = Block::Empty {
                size: filled_block.size(),
            };
            block_disk_map.blocks.remove(empty_index);

            for (i, block) in split.iter().enumerate() {
                block_disk_map.blocks.insert(empty_index + i, block.clone());
            }
        }

        Into::<DiskMap>::into(block_disk_map).checksum().to_string()
    }
}

struct DiskMap {
    blocks: Vec<Option<usize>>,
}

impl DiskMap {
    fn checksum(&self) -> usize {
        self.blocks
            .clone()
            .iter()
            .enumerate()
            .fold(0, |acc, (i, id)| {
                if let Some(id) = id {
                    return acc + i * id;
                }

                acc
            })
    }
}

impl FromStr for DiskMap {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut current_id = 0;

        let blocks: Vec<Option<usize>> = s
            .trim()
            .chars()
            .enumerate()
            .flat_map(|(i, c)| {
                let times: usize = c
                    .to_string()
                    .parse()
                    .unwrap_or_else(|_| panic!("cannot parse char to usize: '{}'", c));

                let value: Option<usize> = match i % 2 == 0 {
                    true => {
                        let id = Some(current_id);

                        current_id += 1;

                        id
                    }
                    false => None,
                };

                vec![value; times]
            })
            .collect();

        Ok(Self { blocks })
    }
}

impl Display for DiskMap {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let value = self
            .blocks
            .iter()
            .map(|v| match v {
                None => '.',
                Some(v) => (v % 10).to_string().chars().next().unwrap(),
            })
            .join("");

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
    fn size(&self) -> usize {
        match self {
            Block::Empty { size } => size.to_owned(),
            Block::Filled { size, .. } => size.to_owned(),
        }
    }

    fn split(&self, other: &Self) -> Vec<Self> {
        match (self, other) {
            (
                Block::Empty { size },
                Block::Filled {
                    size: filled_size,
                    id,
                },
            ) => {
                if filled_size > size {
                    panic!("filled size cannot be greater than empty");
                }

                if filled_size == size {
                    return vec![Block::Filled {
                        size: *filled_size,
                        id: *id,
                    }];
                }

                if filled_size < size {
                    return vec![
                        Block::Filled {
                            size: *filled_size,
                            id: *id,
                        },
                        Block::Empty {
                            size: size - filled_size,
                        },
                    ];
                }

                unreachable!()
            }
            (_, _) => panic!("illegal block"),
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
struct BlockDiskMap {
    blocks: Vec<Block>,
}

impl FromStr for BlockDiskMap {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut current_id = 0;

        let blocks: Vec<Block> = s
            .trim()
            .chars()
            .enumerate()
            .map(|(i, c)| {
                let size: usize = c
                    .to_string()
                    .parse()
                    .unwrap_or_else(|_| panic!("cannot parse char to usize: '{}'", c));

                match i % 2 == 0 {
                    true => {
                        let id = current_id;

                        current_id += 1;

                        Block::Filled { size, id }
                    }
                    false => Block::Empty { size },
                }
            })
            .collect();

        Ok(Self { blocks })
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

        let result = DiskMap::from_str("2333133121414131402").unwrap();
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
                Block::Filled { size: 1, id: 0 },
                Block::Empty { size: 2 },
                Block::Filled { size: 3, id: 1 },
                Block::Empty { size: 4 },
                Block::Filled { size: 5, id: 2 },
            ],
        };

        assert_eq!(expected, sut);
    }
}
