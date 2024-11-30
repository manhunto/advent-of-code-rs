use crate::solutions::year2023::day20::ModuleType::{Broadcaster, Conjunction, FlipFlop};
use crate::solutions::Solution;
use crate::utils::math::lcm;
use itertools::Itertools;
use std::collections::{HashMap, HashSet, VecDeque};

type Modules = HashMap<String, Module>;
type ConjunctionInputs = HashMap<String, Vec<String>>;

pub struct Day20;

impl Solution for Day20 {
    fn part_one(&self, input: &str) -> String {
        let (mut modules, _) = Self::parse_input(input);

        let mut high_pulses: usize = 0;
        let mut low_pulses: usize = 0;

        for _ in 1..=1000 {
            let mut inputs: VecDeque<Input> = VecDeque::from(vec![Input::new(
                "button".to_string(),
                "broadcaster".to_string(),
                Pulse::Low,
            )]);

            low_pulses += 1;

            while let Some(input) = inputs.pop_front() {
                if let Some(module) = modules.get(&input.to) {
                    let (module_type, pulse) = module.process(input.from.clone(), input.pulse);

                    if let Some(pulse) = pulse {
                        match pulse {
                            Pulse::Low => low_pulses += module.destinations.len(),
                            Pulse::High => high_pulses += module.destinations.len(),
                        }

                        for dest in &module.destinations {
                            inputs.push_back(Input::new(input.to.clone(), dest.clone(), pulse));
                        }
                    }

                    *modules.get_mut(&input.to).unwrap() = module.with_type(module_type);
                }
            }
        }

        (high_pulses * low_pulses).to_string()
    }

    fn part_two(&self, input: &str) -> String {
        let (mut modules, conjunction_inputs) = Self::parse_input(input);
        let mut button_click: usize = 0;

        let rx_parent = Self::find_parent(&modules, "rx".to_string());
        let rx_parent_inputs = conjunction_inputs.get(&rx_parent).unwrap();
        let mut first_high_pulse_button_push: HashMap<String, usize> =
            HashMap::with_capacity(rx_parent_inputs.len());

        loop {
            let mut inputs: VecDeque<Input> = VecDeque::from(vec![Input::new(
                "button".to_string(),
                "broadcaster".to_string(),
                Pulse::Low,
            )]);

            button_click += 1;

            while let Some(input) = inputs.pop_front() {
                if rx_parent_inputs.contains(&input.from.clone())
                    && input.to == rx_parent
                    && input.pulse.eq(&Pulse::High)
                {
                    first_high_pulse_button_push
                        .entry(input.from.clone())
                        .or_insert(button_click);

                    if first_high_pulse_button_push.len() == rx_parent_inputs.len() {
                        let high_pulses = first_high_pulse_button_push.values().copied().collect();

                        return lcm(high_pulses).to_string();
                    }
                }

                if let Some(module) = modules.get(&input.to) {
                    let (module_type, pulse) = module.process(input.from.clone(), input.pulse);

                    if let Some(pulse) = pulse {
                        for dest in &module.destinations {
                            inputs.push_back(Input::new(input.to.clone(), dest.clone(), pulse));
                        }
                    }

                    *modules.get_mut(&input.to).unwrap() = module.with_type(module_type);
                }
            }
        }
    }
}

impl Day20 {
    fn parse_input(input: &str) -> (Modules, ConjunctionInputs) {
        let modules: Modules = input
            .lines()
            .map(|line| {
                let (module, dest) = line.split_terminator(" -> ").collect_tuple().unwrap();
                let destinations: Vec<String> =
                    dest.split_terminator(", ").map(String::from).collect();

                match &module[0..1] {
                    "%" => (
                        module[1..].to_string(),
                        Module::new(FlipFlop { status: false }, destinations),
                    ),
                    "&" => (
                        module[1..].to_string(),
                        Module::new(
                            Conjunction {
                                memory: HashMap::new(),
                            },
                            destinations,
                        ),
                    ),
                    "b" => (module.to_string(), Module::new(Broadcaster, destinations)),
                    _ => unreachable!(),
                }
            })
            .collect();

        let conjunction_inputs: ConjunctionInputs = Self::input_for_conjunctions(&modules);

        (
            Self::update_conjunction_inputs(modules, &conjunction_inputs),
            conjunction_inputs,
        )
    }

    fn input_for_conjunctions(modules: &Modules) -> ConjunctionInputs {
        let conjunctions_names: HashSet<&String> = modules
            .iter()
            .filter_map(|(name, m)| match m.module_type {
                Conjunction { .. } => Some(name),
                _ => None,
            })
            .collect();

        let mut conjunction_inputs: ConjunctionInputs =
            HashMap::with_capacity(conjunctions_names.len());

        for (name, module) in modules {
            let inputs: HashSet<&String> = module.destinations.iter().collect();

            for input in inputs.intersection(&conjunctions_names) {
                conjunction_inputs
                    .entry(input.to_string())
                    .or_insert(Vec::with_capacity(4))
                    .push(name.clone());
            }
        }

        conjunction_inputs
    }

    fn update_conjunction_inputs(
        modules: Modules,
        conjunction_inputs: &ConjunctionInputs,
    ) -> Modules {
        let mut modules: Modules = modules.into_iter().collect();

        for (name, inputs) in conjunction_inputs {
            let module = modules.get(name).unwrap();
            let inputs: HashMap<String, Pulse> =
                inputs.iter().map(|i| (i.to_string(), Pulse::Low)).collect();

            *modules.get_mut(name).unwrap() = module.with_type(Conjunction { memory: inputs });
        }

        modules
    }

    fn find_parent(modules: &Modules, dest: String) -> String {
        let vec = modules
            .iter()
            .filter_map(|(name, module)| match module.destinations.contains(&dest) {
                true => Some(name.to_string()),
                false => None,
            })
            .collect::<Vec<String>>();

        vec.first().unwrap().to_string()
    }
}

struct Input {
    from: String,
    to: String,
    pulse: Pulse,
}

impl Input {
    fn new(from: String, to: String, pulse: Pulse) -> Self {
        Self { from, to, pulse }
    }
}

#[derive(Debug)]
struct Module {
    module_type: ModuleType,
    destinations: Vec<String>,
}

impl Module {
    fn new(module_type: ModuleType, destinations: Vec<String>) -> Self {
        Self {
            module_type,
            destinations,
        }
    }

    fn with_type(&self, module_type: ModuleType) -> Self {
        Self {
            module_type,
            destinations: self.destinations.clone(),
        }
    }

    fn process(&self, from: String, pulse: Pulse) -> (ModuleType, Option<Pulse>) {
        self.module_type.process(from, pulse)
    }
}

#[derive(Debug)]
enum ModuleType {
    Broadcaster,
    FlipFlop { status: bool },
    Conjunction { memory: HashMap<String, Pulse> },
}

impl ModuleType {
    fn process(&self, from: String, pulse: Pulse) -> (Self, Option<Pulse>) {
        match self {
            Broadcaster => (Broadcaster, Some(pulse)),
            FlipFlop { status } => match pulse {
                Pulse::High => (FlipFlop { status: *status }, None),
                Pulse::Low => {
                    let new_pulse = match status {
                        true => Pulse::Low,
                        false => Pulse::High,
                    };

                    (FlipFlop { status: !status }, Some(new_pulse))
                }
            },
            Conjunction { memory } => {
                let mut new_memory = memory.clone();
                *new_memory.get_mut(&from).unwrap() = pulse;

                let new = Conjunction {
                    memory: new_memory.clone(),
                };

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
    use crate::solutions::year2023::day20::Day20;
    use crate::solutions::year2023::read_2023_example;
    use crate::solutions::Solution;

    #[test]
    fn part_one_example_test() {
        let input = read_2023_example("20");

        assert_eq!("32000000", Day20.part_one(input.as_str()));
    }

    #[test]
    fn part_one_example_2_test() {
        let input = read_2023_example("20_2");

        assert_eq!("11687500", Day20.part_one(input.as_str()));
    }
}
