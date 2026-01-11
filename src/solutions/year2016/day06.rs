use crate::solutions::Solution;
use std::collections::HashMap;

pub struct Day06;

impl Solution for Day06 {
    fn part_one(&self, input: &str) -> String {
        ColumnFrequencies::new(input)
            .map(|freq_map| {
                freq_map
                    .into_iter()
                    .max_by_key(|(_, count)| *count)
                    .map(|(ch, _)| ch)
                    .unwrap()
            })
            .collect()
    }

    fn part_two(&self, input: &str) -> String {
        ColumnFrequencies::new(input)
            .map(|freq_map| {
                freq_map
                    .into_iter()
                    .min_by_key(|(_, count)| *count)
                    .map(|(ch, _)| ch)
                    .unwrap()
            })
            .collect()
    }
}

struct ColumnFrequencies<'a> {
    lines: Vec<&'a str>,
    current_column: usize,
    num_columns: usize,
}

impl<'a> ColumnFrequencies<'a> {
    fn new(input: &'a str) -> Self {
        let lines: Vec<&str> = input.lines().collect();
        let num_columns = lines.first().map_or(0, |line| line.len());

        Self {
            lines,
            current_column: 0,
            num_columns,
        }
    }
}

impl<'a> Iterator for ColumnFrequencies<'a> {
    type Item = HashMap<char, usize>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current_column >= self.num_columns {
            return None;
        }

        let freq_map = self
            .lines
            .iter()
            .filter_map(|line| line.chars().nth(self.current_column))
            .fold(HashMap::new(), |mut map, ch| {
                *map.entry(ch).or_insert(0) += 1;
                map
            });

        self.current_column += 1;

        Some(freq_map)
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let remaining = self.num_columns - self.current_column;
        (remaining, Some(remaining))
    }
}

impl<'a> ExactSizeIterator for ColumnFrequencies<'a> {}

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
