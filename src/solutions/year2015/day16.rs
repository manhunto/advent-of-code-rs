use crate::solutions::Solution;
use itertools::Itertools;
use std::collections::HashMap;
use std::fmt::Debug;
use std::ops::Add;
use std::str::FromStr;

const TARGET_SUE: &str = r#"children: 3
cats: 7
samoyeds: 2
pomeranians: 3
akitas: 0
vizslas: 0
goldfish: 5
trees: 3
cars: 2
perfumes: 1"#;

const FACT_SEPARATOR: &str = ", ";

pub struct Day16;

impl Solution for Day16 {
    fn part_one(&self, input: &str) -> String {
        let target = self.parse_target_sue();

        self.parse_input(input)
            .position(|sue| sue.matches_part_one(&target))
            .unwrap()
            .add(1)
            .to_string()
    }

    fn part_two(&self, input: &str) -> String {
        let target = self.parse_target_sue();

        self.parse_input(input)
            .position(|sue| sue.matches_part_two(&target))
            .unwrap()
            .add(1)
            .to_string()
    }
}

impl Day16 {
    fn parse_input<'a>(&'a self, input: &'a str) -> impl Iterator<Item = Sue> + 'a {
        input.lines().map(|line| {
            let line = line.split_whitespace().skip(2).join(" ");

            line.parse().unwrap()
        })
    }

    fn parse_target_sue(&self) -> Sue {
        TARGET_SUE.lines().join(FACT_SEPARATOR).parse().unwrap()
    }
}

#[derive(Debug)]
struct Sue {
    children: Option<u8>,
    cats: Option<u8>,
    samoyeds: Option<u8>,
    pomeranians: Option<u8>,
    akitas: Option<u8>,
    vizslas: Option<u8>,
    goldfish: Option<u8>,
    trees: Option<u8>,
    cars: Option<u8>,
    perfumes: Option<u8>,
}

impl Sue {
    fn matches_part_one(&self, target: &Sue) -> bool {
        self.field_equals(self.children, target.children)
            && self.field_equals(self.cats, target.cats)
            && self.field_equals(self.samoyeds, target.samoyeds)
            && self.field_equals(self.pomeranians, target.pomeranians)
            && self.field_equals(self.akitas, target.akitas)
            && self.field_equals(self.vizslas, target.vizslas)
            && self.field_equals(self.goldfish, target.goldfish)
            && self.field_equals(self.trees, target.trees)
            && self.field_equals(self.cars, target.cars)
            && self.field_equals(self.perfumes, target.perfumes)
    }

    fn matches_part_two(&self, target: &Sue) -> bool {
        self.field_equals(self.children, target.children)
            && self.field_greater(self.cats, target.cats)
            && self.field_equals(self.samoyeds, target.samoyeds)
            && self.field_less_than(self.pomeranians, target.pomeranians)
            && self.field_equals(self.akitas, target.akitas)
            && self.field_equals(self.vizslas, target.vizslas)
            && self.field_less_than(self.goldfish, target.goldfish)
            && self.field_greater(self.trees, target.trees)
            && self.field_equals(self.cars, target.cars)
            && self.field_equals(self.perfumes, target.perfumes)
    }

    fn field_equals(&self, own: Option<u8>, target: Option<u8>) -> bool {
        own.is_none() || own == target
    }

    fn field_greater(&self, own: Option<u8>, target: Option<u8>) -> bool {
        own.is_none() || own > target
    }

    fn field_less_than(&self, own: Option<u8>, target: Option<u8>) -> bool {
        own.is_none() || own < target
    }
}

impl FromStr for Sue {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let map: HashMap<&str, u8> = s
            .split(FACT_SEPARATOR)
            .map(|f| {
                let (name, value) = f.split_once(": ").unwrap();

                (name, value.parse().unwrap())
            })
            .collect();

        Ok(Self {
            children: map.get("children").copied(),
            cats: map.get("cats").copied(),
            samoyeds: map.get("samoyeds").copied(),
            pomeranians: map.get("pomeranians").copied(),
            akitas: map.get("akitas").copied(),
            vizslas: map.get("vizslas").copied(),
            goldfish: map.get("goldfish").copied(),
            trees: map.get("trees").copied(),
            cars: map.get("cars").copied(),
            perfumes: map.get("perfumes").copied(),
        })
    }
}
