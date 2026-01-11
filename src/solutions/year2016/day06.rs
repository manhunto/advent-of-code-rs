use crate::solutions::Solution;
use itertools::Itertools;
use std::collections::HashMap;

pub struct Day06;

impl Solution for Day06 {
    fn part_one(&self, input: &str) -> String {
        self.group(input)
            .map(|map| *map.iter().max_by_key(|(_, v)| *v).unwrap().0)
            .collect()
    }

    fn part_two(&self, input: &str) -> String {
        self.group(input)
            .map(|map| *map.iter().min_by_key(|(_, v)| *v).unwrap().0)
            .collect()
    }
}

impl Day06 {
    fn group(&self, input: &str) -> impl Iterator<Item = HashMap<char, usize>> {
        let mut columns: HashMap<usize, HashMap<char, usize>> = HashMap::new();

        for line in input.lines() {
            for (column, char) in line.char_indices() {
                *columns.entry(column).or_default().entry(char).or_insert(0) += 1;
            }
        }

        let sorted: Vec<_> = columns
            .into_iter()
            .sorted_by_key(|(k, _)| *k)
            .map(|(_, map)| map)
            .collect();

        sorted.into_iter()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = r#"eedadn
drvtee
eandsr
raavrd
atevrs
tsrnev
sdttsa
rasrtv
nssdts
ntnada
svetve
tesnvt
vntsnd
vrdear
dvrsen
enarar"#;

    #[test]
    fn part_one_example() {
        assert_eq!("easter", Day06.part_one(EXAMPLE));
    }

    #[test]
    fn part_two_example() {
        assert_eq!("advent", Day06.part_two(EXAMPLE));
    }
}
