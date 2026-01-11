use crate::solutions::Solution;
use itertools::Itertools;
use std::collections::HashMap;

pub struct Day06;

impl Solution for Day06 {
    fn part_one(&self, input: &str) -> String {
        let mut vec: HashMap<usize, HashMap<char, usize>> = HashMap::new();

        for line in input.lines() {
            for (column, char) in line.char_indices() {
                *vec.entry(column).or_default().entry(char).or_insert(0) += 1;
            }
        }

        vec.iter()
            .sorted_by_key(|(k, _)| *k)
            .map(|(_, map)| map.iter().max_by_key(|(_, v)| *v).unwrap().0)
            .collect()
    }

    fn part_two(&self, _input: &str) -> String {
        String::from("0")
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
}
