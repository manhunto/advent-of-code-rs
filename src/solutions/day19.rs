use std::collections::HashMap;
use itertools::Itertools;
use regex::Regex;
use crate::solutions::day19::Action::MoveToWorkflow;
use crate::solutions::day19::Rule::{Conditional, OnlyAction};
use crate::solutions::Solution;

pub struct Day19;

impl Solution for Day19 {
    fn part_one(&self, input: &str) -> String {
        let (parts, workflows) = Self::parse_input(input);

        parts
            .iter()
            .map(|part| {
                let mut workflow_name = "in";

                loop {
                    let workflow = workflows.get(workflow_name).unwrap();
                    let action = workflow.process(part);

                    match action {
                        Action::Accepted => return part.sum(),
                        Action::Rejected => return 0,
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
    fn parse_input(input: &str) -> (Vec<Part>, HashMap<String, Workflow>) {
        let (workflows_string, parts_string) = input.split("\n\n").collect_tuple().unwrap();

        let re = Regex::new(r"^\{x=(\d+),m=(\d+),a=(\d+),s=(\d+)}").unwrap();
        let parts: Vec<Part> = parts_string
            .lines()
            .map(|line| {
                let (_, [x, m, a, s]) = re.captures(line).unwrap().extract();

                Part {
                    x: x.parse().unwrap(),
                    m: m.parse().unwrap(),
                    a: a.parse().unwrap(),
                    s: s.parse().unwrap(),
                }
            })
            .collect();

        let re = Regex::new(r"([a-z]{2,3})\{(.*)}").unwrap();
        let workflows: HashMap<String, Workflow> = workflows_string
            .lines()
            .map(|line| {
                let (_, [name, rules_string]) = re.captures(line).unwrap().extract();

                (name.to_string(), Workflow::from(rules_string))
            })
            .collect();

        (parts, workflows)
    }
}

#[derive(Debug, Clone, Copy)]
struct Part {
    x: isize,
    m: isize,
    a: isize,
    s: isize,
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
                Conditional(condition) => if condition.is_valid(part) {
                    return &condition.action;
                }
                OnlyAction(action) => return action
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
    fn new(category: String, operation: char, value: isize, action: Action) -> Self {
        Self { category, operation, value, action }
    }

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

#[derive(Debug)]
enum Rule {
    Conditional(Condition),
    OnlyAction(Action),
}

impl From<&str> for Rule {
    fn from(value: &str) -> Self {
        if value.contains(':') {
            let (cond_part, action) = value.split_terminator(":").collect_tuple().unwrap();
            let (cat, value) = cond_part.split_terminator(&['<', '>'][..]).collect_tuple().unwrap();
            let operation = if cond_part.contains('<') { '<' } else { '>' };

            return Conditional(Condition::new(
                cat.to_string(),
                operation,
                value.parse().unwrap(),
                Action::from(action),
            ));
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
            "A" => Action::Accepted,
            "R" => Action::Rejected,
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
