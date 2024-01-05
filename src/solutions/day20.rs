use std::collections::{HashMap, VecDeque};
use itertools::Itertools;
use crate::solutions::day20::ModuleType::{Broadcaster, Conjunction, FlipFlop};
use crate::solutions::Solution;

type Modules = HashMap<String, Module>;

pub struct Day20;

impl Solution for Day20 {
    fn part_one(&self, input: &str) -> String {
        let first = Self::first_module(input);
        let mut modules: Modules = Self::parse_input(input);

        let mut high_pulses: usize = 0;
        let mut low_pulses: usize = 0;

        for _ in 1..=1000 {
            let mut inputs: VecDeque<Input> = VecDeque::new();
            inputs.push_back(Input { from: "button".to_string(), to: first.clone(), pulse: Pulse::Low });
            low_pulses += 1;

            while let Some(input) = inputs.pop_front() {
                if let Some(module) = modules.get(&input.to) {
                    let (module_type, pulse) = module.module_type.process(input.from.clone(), input.pulse);

                    if let Some(pulse) = pulse {
                        if pulse == Pulse::High {
                            high_pulses += module.destinations.len();
                        } else {
                            low_pulses += module.destinations.len();
                        }

                        for dest in &module.destinations {
                            inputs.push_back(Input { from: input.to.clone(), to: dest.clone(), pulse })
                        }
                    }

                    *modules.get_mut(&input.to).unwrap() = Module { module_type, destinations: module.destinations.clone() }
                }
            }
        }

        (high_pulses * low_pulses).to_string()
    }

    fn part_two(&self, input: &str) -> String {
        String::from('0')
    }
}

impl Day20 {
    fn first_module(input: &str) -> String {
        let (module, _) = input.lines().nth(0).unwrap().split_terminator(" -> ").collect_tuple().unwrap();

        match &module[0..1] {
            "%" | "&" => module[1..].to_string(),
            "b" => module.to_string(),
            _ => unreachable!()
        }
    }

    fn parse_input(input: &str) -> Modules {
        let mut modules: Modules = input
            .lines()
            .map(|line| {
                let (module, dest) = line.split_terminator(" -> ").collect_tuple().unwrap();
                let destinations: Vec<String> = dest.split_terminator(", ").map(|d| d.to_string()).collect();

                match &module[0..1] {
                    "%" => (module[1..].to_string().to_owned(), Module { module_type: FlipFlop { status: false }, destinations }),
                    "&" => (module[1..].to_string(), Module { module_type: Conjunction { memory: HashMap::new() }, destinations }),
                    "b" => (module.to_string(), Module { module_type: Broadcaster, destinations }),
                    _ => unreachable!()
                }
            })
            .collect();

        let mut conjunctions: HashMap<String, Vec<String>> = modules
            .iter()
            .filter_map(|(name, m)| {
                if let Conjunction { .. } = m.module_type {
                    return Some((name.clone(), Vec::new()));
                }

                None
            })
            .collect();

        let binding = conjunctions.clone();
        let conj_names: Vec<&String> = binding.keys().collect();

        for (name, module) in &modules {
            for destination in &module.destinations {
                for conj_name in &conj_names {
                    if conj_name == &destination {
                        conjunctions.entry(conj_name.to_string()).and_modify(|c| c.push(name.clone()));
                    }
                }
            }
        }

        for (name, inputs) in conjunctions {
            let module = modules.get(&name).unwrap();
            let inputs: HashMap<String, Pulse> = inputs.iter().map(|i| (i.to_string(), Pulse::Low)).collect();

            *modules.get_mut(&name).unwrap() = Module { module_type: Conjunction { memory: inputs }, destinations: module.destinations.clone() };
        }

        modules
    }
}

struct Input {
    from: String,
    to: String,
    pulse: Pulse,
}

#[derive(Debug)]
struct Module {
    module_type: ModuleType,
    destinations: Vec<String>,
}

#[derive(Debug)]
enum ModuleType {
    Broadcaster,
    FlipFlop { status: bool },
    Conjunction { memory: HashMap<String, Pulse> },
}

impl ModuleType {
    fn process(&self, name: String, pulse: Pulse) -> (Self, Option<Pulse>) {
        match self {
            Broadcaster => (Broadcaster, Some(pulse)),
            FlipFlop { status } => {
                if pulse == Pulse::High {
                    (FlipFlop { status: *status }, None)
                } else {
                    let new_type = FlipFlop { status: !status };
                    let new_pulse = match status {
                        true => Pulse::Low,
                        false => Pulse::High,
                    };

                    (new_type, Some(new_pulse))
                }
            }
            Conjunction { memory } => {
                let mut new_memory = memory.clone();
                *new_memory.entry(name).or_insert(Pulse::Low) = pulse;

                let new = Conjunction { memory: new_memory.clone() };
                if new_memory.into_iter().all(|(_, p)| p == Pulse::High) {
                    (new, Some(Pulse::Low))
                } else {
                    (new, Some(Pulse::High))
                }
            }
        }
    }
}

#[derive(Eq, Copy, Clone, PartialEq, Debug)]
enum Pulse {
    Low,
    High,
}

#[cfg(test)]
mod tests {
    use crate::file_system::read_example;
    use crate::solutions::day20::Day20;
    use crate::solutions::Solution;

    #[test]
    fn part_one_example_test() {
        let input = read_example("20");

        assert_eq!("32000000", Day20.part_one(input.as_str()));
    }

    #[test]
    fn part_one_example_2_test() {
        let input = read_example("20_2");

        assert_eq!("11687500", Day20.part_one(input.as_str()));
    }
}
