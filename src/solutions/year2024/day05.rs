use crate::solutions::Solution;
use itertools::Itertools;
use std::cmp::Ordering;

pub struct Day05;

impl Solution for Day05 {
    fn part_one(&self, input: &str) -> String {
        let (page_ordering_rules, page_updates) = self.parse(input);

        page_updates
            .iter()
            .filter(|&update| update.apply_all(&page_ordering_rules))
            .map(|update| update.middle_page())
            .sum::<usize>()
            .to_string()
    }

    fn part_two(&self, input: &str) -> String {
        let (page_ordering_rules, page_updates) = self.parse(input);

        page_updates
            .iter()
            .filter(|&update| !update.apply_all(&page_ordering_rules))
            .map(|update| {
                let mut new_pages = update.pages.clone();

                new_pages.sort_by(|a, b| {
                    let rule = page_ordering_rules
                        .clone()
                        .into_iter()
                        .find(|r| r.first == *a && r.second == *b);

                    match rule {
                        None => Ordering::Greater,
                        Some(_) => Ordering::Less,
                    }
                });

                let new_update = Update { pages: new_pages };

                new_update.middle_page()
            })
            .sum::<usize>()
            .to_string()
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

#[derive(Debug, Clone)]
struct Rule {
    first: usize,
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

impl Update {
    fn apply_all(&self, rules: &[Rule]) -> bool {
        rules.iter().all(|rule| self.apply(rule))
    }

    fn apply(&self, rule: &Rule) -> bool {
        let first_pos = self.pages.iter().position(|&x| x == rule.first);
        let second_pos = self.pages.iter().position(|&x| x == rule.second);

        match (first_pos, second_pos) {
            (Some(first), Some(second)) => first < second,
            _ => true,
        }
    }

    fn middle_page(&self) -> usize {
        let mid = self.pages.len() / 2;

        *self.pages.get(mid).unwrap()
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
    fn part_one_example_test() {
        assert_eq!("143", Day05.part_one(EXAMPLE));
    }

    #[test]
    fn part_second_example_test() {
        assert_eq!("123", Day05.part_two(EXAMPLE));
    }
}
