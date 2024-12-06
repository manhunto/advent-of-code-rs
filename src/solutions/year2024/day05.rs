use crate::solutions::Solution;
use itertools::Itertools;

pub struct Day05;

impl Solution for Day05 {
    fn part_one(&self, input: &str) -> String {
        let (page_ordering_rules, page_updates) = self.parse(input);

        println!("{:?}", page_ordering_rules);
        println!("{:?}", page_updates);

        String::from('0')
    }

    fn part_two(&self, _input: &str) -> String {
        String::from('0')
    }
}

impl Day05 {
    fn parse(&self, input: &str) -> (Vec<Rule>, Vec<Update>) {
        let (page_ordering_rules, page_updates) = input.split_once("\n\n").unwrap();

        let page_ordering_rules = page_ordering_rules.lines().map(Rule::from).collect_vec();
        let page_updates = page_updates.lines().map(Update::from).collect_vec();

        (page_ordering_rules, page_updates)
    }
}

#[derive(Debug)]
struct Rule {
    #[allow(dead_code)]
    first: usize,
    #[allow(dead_code)]
    second: usize,
}

impl From<&str> for Rule {
    fn from(value: &str) -> Self {
        let (first, second) = value.split_once("|").unwrap();

        Self {
            first: first.parse().unwrap(),
            second: second.parse().unwrap(),
        }
    }
}

#[derive(Debug)]
struct Update {
    #[allow(dead_code)]
    pages: Vec<usize>,
}

impl From<&str> for Update {
    fn from(value: &str) -> Self {
        let pages = value
            .split_terminator(',')
            .map(|x| x.parse::<usize>().unwrap())
            .collect_vec();

        Self { pages }
    }
}

#[cfg(test)]
mod tests {
    use crate::solutions::year2024::day05::Day05;
    use crate::solutions::Solution;

    const EXAMPLE: &str = r#"47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47"#;

    #[test]
    #[ignore]
    fn part_one_example_test() {
        assert_eq!("143", Day05.part_one(EXAMPLE));
    }
}
