use crate::solutions::Solution;
use crate::utils::binary::Binary;
use itertools::Itertools;
use std::collections::{HashMap, VecDeque};
use std::fmt::{Debug, Formatter};

pub struct Day10;

impl Solution for Day10 {
    fn part_one(&self, input: &str) -> String {
        self.parse_first_part(input)
            .map(|machine| self.fewest_presses(&machine))
            .sum::<usize>()
            .to_string()
    }

    fn part_two(&self, input: &str) -> String {
        self.parse_second_part(input)
            .map(|machine| {
                let buttons = machine.button_wiring;
                let expected_joltage = machine.joltage_requirements;
                let current_joltage: HashMap<usize, usize> = HashMap::new();

                let mut stack: VecDeque<(HashMap<usize, usize>, Vec<usize>, usize)> =
                    VecDeque::new();

                for button in &buttons {
                    stack.push_back((current_joltage.clone(), button.clone(), 1));
                }

                while let Some((current, button, depth)) = stack.pop_front() {
                    let mut new_current = current.clone();
                    for b in button {
                        new_current.entry(b).and_modify(|v| *v += 1).or_insert(1);
                    }

                    let found = expected_joltage
                        .iter()
                        .enumerate()
                        .all(|(i, e)| new_current.get(&i).is_some_and(|v| v == e));

                    if found {
                        return depth;
                    }

                    let exceed = expected_joltage
                        .iter()
                        .enumerate()
                        .any(|(i, e)| new_current.get(&i).is_some_and(|v| v > e));

                    if exceed {
                        continue;
                    }

                    for button in &buttons {
                        stack.push_back((new_current.clone(), button.to_vec(), depth + 1));
                    }
                }

                0
            })
            .sum::<usize>()
            .to_string()
    }
}

impl Day10 {
    fn parse_first_part<'a>(&self, input: &'a str) -> impl Iterator<Item = Machine> + 'a {
        input.lines().map(|line| {
            let mut vec: VecDeque<&str> = line.split_whitespace().collect();

            let first = vec.pop_front().unwrap();
            let _ = vec.pop_back().unwrap();

            let lights_str = first.trim_matches(|c| c == '[' || c == ']');
            let light_diagram: Binary = lights_str
                .chars()
                .map(|c| match c {
                    '.' => 0,
                    '#' => 1,
                    _ => panic!("Invalid char"),
                })
                .join("")
                .parse()
                .unwrap();

            let number_of_lights = lights_str.len();
            let button_wiring: Vec<Binary> = vec
                .iter()
                .map(|str| {
                    str.trim_matches(|c| c == '(' || c == ')')
                        .split_terminator(',')
                        .map(|c| c.parse::<usize>().unwrap())
                        .fold(0usize, |acc, v| acc + (1 << (number_of_lights - 1 - v)))
                        .into()
                })
                .collect();

            Machine::new(light_diagram, button_wiring)
        })
    }

    fn parse_second_part<'a>(&self, input: &'a str) -> impl Iterator<Item = MachineP2> + 'a {
        input.lines().map(|line| {
            let mut vec: VecDeque<&str> = line.split_whitespace().collect();

            let _ = vec.pop_front().unwrap();
            let last = vec.pop_back().unwrap();

            let button_wiring = vec
                .iter()
                .map(|str| {
                    str.trim_matches(|c| c == '(' || c == ')')
                        .split_terminator(',')
                        .map(|c| c.parse::<usize>().unwrap())
                        .collect()
                })
                .collect();

            let joltage_requirements = last
                .trim_matches(|c| c == '{' || c == '}')
                .split_terminator(',')
                .map(|c| c.parse::<usize>().unwrap())
                .collect_vec();

            MachineP2::new(button_wiring, joltage_requirements)
        })
    }

    fn fewest_presses(&self, machine: &Machine) -> usize {
        let light = machine.light_diagram;
        let buttons_wiring = machine.button_wiring.clone();

        let mut stack: VecDeque<(Binary, Binary, usize)> = VecDeque::new();
        for wiring in &buttons_wiring {
            stack.push_back((light, *wiring, 1));
        }

        while let Some((light, wiring, depth)) = stack.pop_front() {
            let new_value = light.get() ^ wiring.get();

            if new_value == 0 {
                return depth;
            }

            let new_light: Binary = new_value.into();
            for wiring in &buttons_wiring {
                stack.push_back((new_light, *wiring, depth + 1));
            }
        }

        unreachable!()
    }
}

struct Machine {
    light_diagram: Binary,
    button_wiring: Vec<Binary>,
}

impl Machine {
    fn new(light_diagram: Binary, button_wiring: Vec<Binary>) -> Self {
        Self {
            light_diagram,
            button_wiring,
        }
    }
}

impl Debug for Machine {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "[{}] {}",
            self.light_diagram,
            self.button_wiring
                .iter()
                .map(|b| format!("({})", b))
                .join(" ")
        )
    }
}

struct MachineP2 {
    button_wiring: Vec<Vec<usize>>,
    joltage_requirements: Vec<usize>,
}

impl MachineP2 {
    fn new(button_wiring: Vec<Vec<usize>>, joltage_requirements: Vec<usize>) -> Self {
        Self {
            button_wiring,
            joltage_requirements,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = r#"[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}
[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}
[.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}"#;

    #[test]
    fn part_one_example_test() {
        assert_eq!("7", Day10.part_one(EXAMPLE));
    }

    // #[test]
    // fn part_two_example_test() {
    //     assert_eq!("33", Day10.part_two(EXAMPLE));
    // }
}
