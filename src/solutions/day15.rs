use std::collections::HashMap;
use std::ops::Mul;
use crate::solutions::Solution;

pub struct Day15;

impl Solution for Day15 {
    fn part_one(&self, input: &str) -> String {
        let steps: Vec<&str> = input.split_terminator(',').collect();

        steps
            .into_iter()
            .map(Day15::hash)
            .sum::<usize>()
            .to_string()
    }

    fn part_two(&self, input: &str) -> String {
        let steps: Vec<&str> = input.split_terminator(',').collect();
        let mut boxes: HashMap<usize, Vec<Lens>> = HashMap::with_capacity(steps.len());

        for step in steps {
            let lens = Lens::try_from(step).unwrap();
            let box_number = Self::hash(lens.label.as_str());

            let current_box = boxes.entry(box_number).or_insert(vec![]);

            if lens.operation == Operation::Equal {
                if let Some(position) = current_box.iter().position(|l| l.label == lens.label) {
                    *current_box.get_mut(position).unwrap() = lens;
                } else {
                    current_box.push(lens);
                }
            } else {
                if let Some(position) = current_box.iter().position(|l| l.label == lens.label) {
                    current_box.remove(position);
                }
            }
        }

        boxes
            .iter()
            .map(|(i, current_box)| {
                current_box
                    .iter()
                    .enumerate()
                    .map(|(p, lens)| (i + 1) * (p + 1) * lens.focal_length)
                    .sum::<usize>()
            })
            .sum::<usize>()
            .to_string()
    }
}

impl Day15 {
    fn hash(step: &str) -> usize {
        step
            .chars()
            .fold(0, |current, char| (current + char as usize).mul(17) % 256)
    }
}

#[derive(Debug, PartialEq, Clone)]
enum Operation {
    Dash,
    Equal,
}

#[derive(Debug, Clone)]
struct Lens {
    label: String,
    operation: Operation,
    focal_length: usize,
}

impl TryFrom<&str> for Lens {
    type Error = String;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        if value.contains('=') {
            let vec = value.split_terminator('=').collect::<Vec<&str>>();
            let mut parts = vec.iter();

            return Ok(Self {
                label: parts.next().unwrap().to_string(),
                focal_length: parts.next().unwrap().parse().unwrap(),
                operation: Operation::Equal,
            });
        } else if value.contains('-') {
            let without_dash = value.replace('-', "");
            return Ok(Self {
                label: without_dash,
                focal_length: 0,
                operation: Operation::Dash,
            });
        }

        Err(String::from("Unrecognized operation"))
    }
}


#[cfg(test)]
mod tests {
    use crate::file_system::read_example;
    use crate::solutions::day15::Day15;
    use crate::solutions::Solution;

    #[test]
    fn part_one_example_test() {
        let input = read_example("15");

        assert_eq!("1320", Day15.part_one(&input.as_str()));
    }

    #[test]
    fn part_two_example_test() {
        let input = read_example("15");

        assert_eq!("145", Day15.part_two(&input.as_str()));
    }

    #[test]
    fn hash_test() {
        assert_eq!(52, Day15::hash("HASH"));
        assert_eq!(30, Day15::hash("rn=1"));
        assert_eq!(253, Day15::hash("cm-"));
        assert_eq!(97, Day15::hash("qp=3"));
        assert_eq!(14, Day15::hash("qp-"));
        assert_eq!(180, Day15::hash("pc=4"));
        assert_eq!(9, Day15::hash("ot=9"));
        assert_eq!(197, Day15::hash("ab=5"));
        assert_eq!(48, Day15::hash("pc-"));
        assert_eq!(214, Day15::hash("pc=6"));
        assert_eq!(231, Day15::hash("ot=7"));

        assert_eq!(0, Day15::hash("rn"));
        assert_eq!(0, Day15::hash("cm"));
        assert_eq!(1, Day15::hash("qp"));
        assert_eq!(3, Day15::hash("pc"));
        assert_eq!(3, Day15::hash("ot"));
        assert_eq!(3, Day15::hash("ab"));
    }
}
