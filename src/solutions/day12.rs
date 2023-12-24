use crate::solutions::Solution;

pub struct Day12;

impl Solution for Day12 {
    fn part_one(&self, input: &str) -> String {
        let records = Self::parse_input(input);

        println!("{:?}", records);

        String::from("0")
    }

    fn part_two(&self, input: &str) -> String {
        String::from("0")
    }
}

impl Day12 {
    fn parse_input(input: &str) -> Vec<ConditionRecord>{
        input
            .lines()
            .map(Self::parse_line)
            .collect()
    }

    fn parse_line(line: &str) -> ConditionRecord {
        let mut groups = line.split_whitespace();

        let pattern: Vec<Spring> = groups.next().unwrap().chars().map(Spring::from).collect();
        let order: Vec<i32> = groups.next().unwrap().chars().filter_map(|c| c.to_string().parse().ok()).collect();

        ConditionRecord {
            pattern,
            order
        }
    }
}

#[derive(Debug)]
struct ConditionRecord {
    pattern: Vec<Spring>,
    order: Vec<i32>
}

#[derive(Debug)]
enum Spring {
    Operational,
    Damaged,
    Unknown
}

impl From<char> for Spring {
    fn from(value: char) -> Self {
        match value {
            '#' => Self::Damaged,
            '.' => Self::Operational,
            '?' => Self::Unknown,
            _ => panic!("Could not resolve spring")
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::file_system::read_example;
    use crate::solutions::day12::Day12;
    use crate::solutions::Solution;

    #[test]
    fn part_one_example_test() {
        let input = read_example("12");

        assert_eq!("21", Day12.part_one(&input.as_str()));
    }
}
