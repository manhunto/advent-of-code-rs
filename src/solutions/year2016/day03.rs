use crate::solutions::Solution;

pub struct Day03;

impl Solution for Day03 {
    fn part_one(&self, input: &str) -> String {
        self.parse(input)
            .filter(|lengths| self.is_valid_triangle(lengths))
            .count()
            .to_string()
    }

    fn part_two(&self, input: &str) -> String {
        self.parse_vertically(input)
            .filter(|lengths| self.is_valid_triangle(lengths))
            .count()
            .to_string()
    }
}

impl Day03 {
    fn parse<'a>(&'a self, input: &'a str) -> impl Iterator<Item = [u16; 3]> + 'a {
        input.lines().map(|line| self.parse_line(line))
    }

    fn parse_vertically(&self, input: &str) -> impl Iterator<Item = [u16; 3]> + '_ {
        let mut all_triangles = Vec::new();
        let mut cols = [const { Vec::new() }; 3];

        for line in input.lines() {
            let [a, b, c] = self.parse_line(line);
            cols[0].push(a);
            cols[1].push(b);
            cols[2].push(c);
        }

        for col in &cols {
            for chunk in col.chunks_exact(3) {
                all_triangles.push([chunk[0], chunk[1], chunk[2]]);
            }
        }

        all_triangles.into_iter()
    }

    fn parse_line(&self, line: &str) -> [u16; 3] {
        let mut nums = line.split_whitespace().map(|s| s.parse::<u16>().unwrap());

        [
            nums.next().unwrap(),
            nums.next().unwrap(),
            nums.next().unwrap(),
        ]
    }

    fn is_valid_triangle(&self, lengths: &[u16; 3]) -> bool {
        let mut l = *lengths;
        l.sort_unstable();

        l[2] < l[0] + l[1]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_valid_triangle() {
        assert!(!Day03.is_valid_triangle(&[5, 10, 25]));
        assert!(!Day03.is_valid_triangle(&[5, 10, 15]));
        assert!(Day03.is_valid_triangle(&[5, 10, 14]));
    }
}
