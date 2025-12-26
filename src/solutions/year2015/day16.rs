use crate::solutions::Solution;
use itertools::Itertools;
use std::collections::HashMap;
use std::fmt::Debug;
use std::ops::Add;
use std::str::FromStr;

const SUE_TO_FIND: &str = r#"children: 3
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
        let target: Sue = SUE_TO_FIND.lines().join(FACT_SEPARATOR).parse().unwrap();

        self.parse(input)
            .position(|sue| sue.matches(&target))
            .unwrap()
            .add(1)
            .to_string()
    }

    fn part_two(&self, _input: &str) -> String {
        String::from("0")
    }
}

impl Day16 {
    fn parse<'a>(&'a self, input: &'a str) -> impl Iterator<Item = Sue> + 'a {
        input.lines().map(|line| {
            let line = line.split_whitespace().skip(2).join(" ");

            line.parse().unwrap()
        })
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
    fn matches(&self, target: &Sue) -> bool {
        self.matches_field(self.children, target.children)
            && self.matches_field(self.cats, target.cats)
            && self.matches_field(self.samoyeds, target.samoyeds)
            && self.matches_field(self.pomeranians, target.pomeranians)
            && self.matches_field(self.akitas, target.akitas)
            && self.matches_field(self.vizslas, target.vizslas)
            && self.matches_field(self.goldfish, target.goldfish)
            && self.matches_field(self.trees, target.trees)
            && self.matches_field(self.cars, target.cars)
            && self.matches_field(self.perfumes, target.perfumes)
    }

    fn matches_field(&self, own: Option<u8>, target: Option<u8>) -> bool {
        own.is_none() || own == target
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
