use crate::solutions::Solution;
use itertools::Itertools;

pub struct Day25;

impl Solution for Day25 {
    fn part_one(&self, input: &str) -> String {
        let (row, col) = self.parse(input);

        InfinityPaper::new()
            .into_iter()
            .find(|(r, c, _)| *r == row && *c == col)
            .unwrap()
            .2
            .to_string()
    }

    fn part_two(&self, _input: &str) -> String {
        String::from("0")
    }
}

impl Day25 {
    fn parse(&self, input: &str) -> (usize, usize) {
        let parts = input.split_whitespace().collect_vec();
        let row: usize = parts[15].trim_end_matches(',').parse().unwrap();
        let col: usize = parts[17].trim_end_matches('.').parse().unwrap();

        (row, col)
    }
}

struct InfinityPaper {
    row: usize,
    col: usize,
    value: usize,
}

impl InfinityPaper {
    fn new() -> Self {
        Self {
            row: 1,
            col: 1,
            value: 20151125,
        }
    }
}

impl Iterator for InfinityPaper {
    type Item = (usize, usize, usize);

    fn next(&mut self) -> Option<Self::Item> {
        let current = (self.row, self.col, self.value);

        self.value = self.value * 252533 % 33554393;

        if self.row == 1 {
            self.row = self.col + 1;
            self.col = 1;
        } else {
            self.row -= 1;
            self.col += 1;
        }

        Some(current)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn infinity_paper() {
        let mut infinity = InfinityPaper::new();

        assert_eq!(Some((1, 1, 20151125)), infinity.next());
        assert_eq!(Some((2, 1, 31916031)), infinity.next());
        assert_eq!(Some((1, 2, 18749137)), infinity.next());
        assert_eq!(Some((3, 1, 16080970)), infinity.next());
        assert_eq!(Some((2, 2, 21629792)), infinity.next());
        assert_eq!(Some((1, 3, 17289845)), infinity.next());
        assert_eq!(Some((4, 1, 24592653)), infinity.next());
        assert_eq!(Some((3, 2, 8057251)), infinity.next());
        assert_eq!(Some((2, 3, 16929656)), infinity.next());
        assert_eq!(Some((1, 4, 30943339)), infinity.next());
    }

    const INPUT: &str = "To continue, please consult the code grid in the manual.  Enter the code at row 6, column 6.";

    #[test]
    fn part_one() {
        assert_eq!(Day25.part_one(INPUT), "27995004");
    }
}
