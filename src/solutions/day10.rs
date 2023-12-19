use crate::solutions::Solution;

pub struct Day10;

impl Solution for Day10 {
    fn part_one(&self, input: &str) -> String {
        let tiles = self.parse_input(&input);

        String::from("0")
    }

    fn part_two(&self, input: &str) -> String {
        String::from("0")
    }
}

impl Day10 {
    fn parse_input(&self, input: &str) -> Vec<Vec<Tile>> {
        input
            .lines()
            .map(|line| {
                line
                    .chars()
                    .map(|c| Tile::from(c))
                    .collect()
            })
            .collect()
    }
}

#[derive(Debug)]
enum Tile {
    NS,
    EW,
    NE,
    NW,
    SW,
    SE,
    Ground,
    Start,
}

impl Tile {
    fn from(char: char) -> Self {
        match char {
            '|' => Self::NS,
            '-' => Self::EW,
            'L' => Self::NE,
            'J' => Self::NW,
            '7' => Self::SW,
            'F' => Self::SE,
            'S' => Self::Start,
            '.' => Self::Ground,
            _ => panic!("{}", format!("Unknown tile: {}", char))
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::file_system::read_example;
    use crate::solutions::day10::Day10;
    use crate::solutions::Solution;

    #[test]
    fn part_one_example_test() {
        let input = read_example("10");

        assert_eq!("4", Day10.part_one(&input.as_str()));
    }
}
