use crate::solutions::Solution;
use itertools::Itertools;
use std::collections::VecDeque;
use std::fmt::{Debug, Display, Formatter};
use std::str::FromStr;

pub struct Day10;

impl Solution for Day10 {
    fn part_one(&self, input: &str) -> String {
        self.parse(input)
            .map(|machine| self.fewest_presses(&machine))
            .sum::<usize>()
            .to_string()
    }

    fn part_two(&self, _input: &str) -> String {
        String::from("0")
    }
}

impl Day10 {
    fn parse<'a>(&self, input: &'a str) -> impl Iterator<Item = Machine> + 'a {
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

    fn fewest_presses(&self, machine: &Machine) -> usize {
        let light = machine.light_diagram;
        let buttons_wiring = machine.button_wiring.clone();

        let mut stack: VecDeque<(Binary, Binary, usize)> = VecDeque::new();
        for wiring in &buttons_wiring {
            stack.push_back((light, *wiring, 1));
        }

        while let Some((light, wiring, depth)) = stack.pop_front() {
            let new_value = light.0 ^ wiring.0;

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

#[derive(Clone, Copy)]
struct Binary(usize);

impl Display for Binary {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:b}", self.0)
    }
}

impl FromStr for Binary {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if !s.chars().all(|b| b == '0' || b == '1') {
            return Err(format!("{} is not a binary number", s));
        }

        let usize = usize::from_str_radix(s, 2).unwrap();

        Ok(Self(usize))
    }
}

impl From<usize> for Binary {
    fn from(value: usize) -> Self {
        Self(value)
    }
}

impl From<Binary> for usize {
    fn from(value: Binary) -> Self {
        value.0
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
}
