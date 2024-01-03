use std::collections::HashMap;
use itertools::Itertools;
use regex::Regex;
use Action::{Accepted, Rejected};
use crate::solutions::day19::Action::MoveToWorkflow;
use crate::solutions::day19::Rule::{Conditional, OnlyAction};
use crate::solutions::Solution;

pub struct Day19;

impl Solution for Day19 {
    fn part_one(&self, input: &str) -> String {
        let parts = Self::parse_parts(input);
        let workflows = Self::parse_workflows(input);

        parts
            .iter()
            .map(|part| {
                let mut workflow_name = "in";

                loop {
                    let workflow = workflows.get(workflow_name).unwrap();
                    let action = workflow.process(part);

                    match action {
                        Accepted => return part.sum(),
                        Rejected => return 0,
                        MoveToWorkflow(workflow) => workflow_name = workflow.as_str()
                    }
                }
            })
            .sum::<isize>()
            .to_string()
    }

    fn part_two(&self, input: &str) -> String {
        String::from('0')
    }
}

impl Day19 {
    fn parse_workflows(input: &str) -> HashMap<String, Workflow> {
        let (workflows_string, _) = input.split("\n\n").collect_tuple().unwrap();
        let re = Regex::new(r"([a-z]{2,3})\{(.*)}").unwrap();

        workflows_string
            .lines()
            .map(|line| {
                let (_, [name, rules_string]) = re.captures(line).unwrap().extract();

                (name.to_string(), Workflow::from(rules_string))
            })
            .collect()
    }

    fn parse_parts(input: &str) -> Vec<Part> {
        let (_, parts_string) = input.split("\n\n").collect_tuple().unwrap();
        let re = Regex::new(r"^\{x=(\d+),m=(\d+),a=(\d+),s=(\d+)}").unwrap();

        parts_string
            .lines()
            .map(|line| {
                let (_, categories) = re.captures(line).unwrap().extract();

                Part::from(categories)
            })
            .collect()
    }
}

#[derive(Debug, Clone, Copy)]
struct Part {
    x: isize,
    m: isize,
    a: isize,
    s: isize,
}

impl From<[&str; 4]> for Part {
    fn from(value: [&str; 4]) -> Self {
        let [x, m, a, s] = value;

        Self {
            x: x.parse().unwrap(),
            m: m.parse().unwrap(),
            a: a.parse().unwrap(),
            s: s.parse().unwrap(),
        }
    }
}

impl Part {
    fn sum(&self) -> isize {
        self.x + self.m + self.a + self.s
    }
}

struct Workflow {
    rules: Vec<Rule>,
}

impl Workflow {
    fn process(&self, part: &Part) -> &Action {
        let mut iter = self.rules.iter();

        while let Some(rule) = iter.next() {
            match rule {
                Conditional(condition) if condition.is_valid(part) => return &condition.action,
                OnlyAction(action) => return action,
                _ => {}
            }
        }

        unreachable!()
    }
}

impl From<&str> for Workflow {
    fn from(value: &str) -> Self {
        let conditions_vec: Vec<&str> = value.split_terminator(",").collect();

        let rules: Vec<Rule> = conditions_vec
            .into_iter()
            .map(Rule::from)
            .collect();

        Self { rules }
    }
}

#[derive(Debug)]
struct Condition {
    category: String,
    operation: char,
    value: isize,
    action: Action,
}

impl Condition {
    fn is_valid(&self, part: &Part) -> bool {
        let part_value = match self.category.as_str() {
            "x" => part.x,
            "m" => part.m,
            "a" => part.a,
            "s" => part.s,
            _ => unreachable!()
        };

        return match self.operation {
            '<' => part_value < self.value,
            '>' => part_value > self.value,
            _ => unreachable!()
        };
    }
}

impl From<&str> for Condition {
    fn from(value: &str) -> Self {
        let (cond_part, action) = value.split_terminator(':').collect_tuple().unwrap();
        let (cat, value) = cond_part.split_terminator(&['<', '>'][..]).collect_tuple().unwrap();
        let operation = if cond_part.contains('<') { '<' } else { '>' };

        Self {
            category: cat.to_string(),
            operation,
            value: value.parse().unwrap(),
            action: Action::from(action)
        }
    }
}

#[derive(Debug)]
enum Rule {
    Conditional(Condition),
    OnlyAction(Action),
}

impl From<&str> for Rule {
    fn from(value: &str) -> Self {
        if value.contains(':') {
            return Conditional(Condition::from(value));
        }

        return OnlyAction(Action::from(value));
    }
}

#[derive(Debug)]
enum Action {
    Accepted,
    Rejected,
    MoveToWorkflow(String),
}

impl From<&str> for Action {
    fn from(value: &str) -> Self {
        match value {
            "A" => Accepted,
            "R" => Rejected,
            _ => MoveToWorkflow(value.to_string())
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::file_system::read_example;
    use crate::solutions::day19::Day19;
    use crate::solutions::Solution;

    #[test]
    fn part_one_example_test() {
        let input = read_example("19");

        assert_eq!("19114", Day19.part_one(&input.as_str()));
    }
}
