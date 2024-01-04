use std::collections::HashMap;
use std::slice::Iter;
use itertools::Itertools;
use regex::Regex;
use Action::{Accepted, Rejected};
use crate::range::Range;
use crate::solutions::day19::Action::MoveToWorkflow;
use crate::solutions::day19::Rule::{Conditional, Actionable};
use crate::solutions::Solution;

type Workflows = HashMap<String, Workflow>;

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
        let workflows = Self::parse_workflows(input);

        Self::combinations(&workflows, 4000).to_string()
    }
}

impl Day19 {
    fn parse_workflows(input: &str) -> Workflows {
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

    fn combinations(workflows: &Workflows, range_to: usize) -> usize {
        let part_ranges = PartRanges::new(range_to);

        Self::combinations_for(workflows, "in", part_ranges, range_to)
    }

    fn combinations_for(workflows: &Workflows, name: &str, part_ranges: PartRanges, range_to: usize) -> usize {
        let workflow = workflows.get(name).unwrap();

        let mut combinations: usize = 0;
        let iter: Iter<Rule> = workflow.rules.iter();
        let mut part_ranges = part_ranges;

        for rule in iter {
            match rule {
                Conditional(condition) => {
                    let Condition { operation, value, category, action } = condition;
                    let true_range = match operation {
                        '>' => Range::new(*value as i64 + 1, range_to as i64).unwrap(),
                        '<' => Range::new(1, *value as i64 - 1).unwrap(),
                        _ => unreachable!()
                    };

                    let true_ranges = part_ranges.intersect(category, &true_range);
                    if true_ranges.is_some() {
                        match action {
                            Accepted => combinations += true_ranges.unwrap().combinations(),
                            Rejected => {}
                            MoveToWorkflow(workflow) => {
                                combinations += Self::combinations_for(workflows, workflow.as_str(), true_ranges.unwrap(), range_to);
                            }
                        };
                    }

                    let false_range = match operation {
                        '<' => Range::new(*value as i64, range_to as i64).unwrap(),
                        '>' => Range::new(1, *value as i64).unwrap(),
                        _ => unreachable!()
                    };

                    let false_ranges = part_ranges.intersect(category, &false_range);
                    if false_ranges.is_none() {
                        break;
                    }
                    part_ranges = part_ranges.intersect(category, &false_range).unwrap();
                }
                Actionable(action) => match action {
                    Accepted => combinations += part_ranges.combinations(),
                    Rejected => {
                        break;
                    }
                    MoveToWorkflow(workflow) => {
                        combinations += Self::combinations_for(workflows, workflow.as_str(), part_ranges, range_to);
                        break;
                    }
                },
            }
        }


        combinations
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

#[derive(Clone)]
struct PartRanges {
    x: Range,
    m: Range,
    a: Range,
    s: Range,
}

impl PartRanges {
    fn new(to: usize) -> Self {
        Self {
            x: Range::new(1, to as i64).unwrap(),
            m: Range::new(1, to as i64).unwrap(),
            a: Range::new(1, to as i64).unwrap(),
            s: Range::new(1, to as i64).unwrap(),
        }
    }

    fn combinations(&self) -> usize {
        self.x.len() as usize
            * self.m.len() as usize
            * self.a.len() as usize
            * self.s.len() as usize
    }

    fn intersect(&self, category: &str, range: &Range) -> Option<Self> {
        let is_allowed_to_sub = match category {
            "x" => self.x.collide(range),
            "m" => self.m.collide(range),
            "a" => self.a.collide(range),
            "s" => self.s.collide(range),
            _ => unreachable!()
        };

        if !is_allowed_to_sub {
            return None;
        }

        Some(match category {
            "x" => Self { x: self.x.intersect(range).unwrap(), m: self.m, a: self.a, s: self.s },
            "m" => Self { m: self.m.intersect(range).unwrap(), x: self.x, a: self.a, s: self.s },
            "a" => Self { a: self.a.intersect(range).unwrap(), m: self.m, x: self.x, s: self.s },
            "s" => Self { s: self.s.intersect(range).unwrap(), m: self.m, a: self.a, x: self.x },
            _ => unreachable!()
        })
    }
}

#[derive(Debug)]
struct Workflow {
    rules: Vec<Rule>,
}

impl Workflow {
    fn process(&self, part: &Part) -> &Action {
        let iter: Iter<Rule> = self.rules.iter();

        for rule in iter {
            match rule {
                Conditional(condition) if condition.is_valid(part) => return &condition.action,
                Actionable(action) => return action,
                _ => {}
            }
        }

        unreachable!()
    }
}

impl From<&str> for Workflow {
    fn from(value: &str) -> Self {
        let conditions_vec: Vec<&str> = value.split_terminator(',').collect();

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

        match self.operation {
            '<' => part_value < self.value,
            '>' => part_value > self.value,
            _ => unreachable!()
        }
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
            action: Action::from(action),
        }
    }
}

#[derive(Debug)]
enum Rule {
    Conditional(Condition),
    Actionable(Action),
}

impl From<&str> for Rule {
    fn from(value: &str) -> Self {
        if value.contains(':') {
            return Conditional(Condition::from(value));
        }

        Actionable(Action::from(value))
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
    use crate::solutions::day19::{Day19, Workflow, Workflows};
    use crate::solutions::Solution;

    macro_rules! assert_combinations {
        ($result: expr, $workflows: expr) => {
            let workflows = create_workflows($workflows);

            assert_eq!($result, Day19::combinations(&workflows, 5))
    };
}

    #[test]
    fn part_one_example_test() {
        let input = read_example("19");

        assert_eq!("19114", Day19.part_one(input.as_str()));
    }

    #[test]
    fn part_two_example_test() {
        let input = read_example("19");

        assert_eq!("167409079868000", Day19.part_two(input.as_str()));
    }

    #[test]
    fn combinations_test() {
        assert_combinations!(5*5*5*5, vec![("in", "a<3:A,A")]);
        assert_combinations!(1*5*5*5, vec![("in", "a<2:A,R")]);
        assert_combinations!(2*2*5*5, vec![("in", "a<3:dfg,R"), ("dfg", "x>3:A,R")]);
        assert_combinations!(1*5*5*5, vec![("in", "a<4:dfg,R"), ("dfg", "a>2:A,R")]);
        assert_combinations!(0 ,vec![("in", "a<4:dfg,R"), ("dfg", "a>2:R,R")]);
        assert_combinations!(1*1*5*5 ,vec![("in", "a<2:dfg,R"), ("dfg", "x>4:A,R")]);
        assert_combinations!(1*3*5*5, vec![("in", "a>4:dfg,R"), ("dfg", "a>2:cfg,R"), ("cfg", "m<3:R,A")]);
        assert_combinations!(2*5*5*5 + 3*2*5*5, vec![("in", "a>2:dfg,cfg"), ("dfg", "a>3:A,R"), ("cfg", "m<3:R,A")]);
    }

    fn create_workflows(input: Vec<(&str, &str)>) -> Workflows {
        input
            .iter()
            .map(|(name, conditions)| {
                (name.to_string(), Workflow::from(*conditions))
            })
            .collect()
    }
}