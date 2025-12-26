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
        let sue_to_find: Sue = SUE_TO_FIND.lines().join(FACT_SEPARATOR).parse().unwrap();

        self.parse(input)
            .find_position(|sue| {
                sue.children
                    .is_none_or(|x| x == sue_to_find.children.unwrap())
                    && sue.cats.is_none_or(|x| x == sue_to_find.cats.unwrap())
                    && sue
                        .samoyeds
                        .is_none_or(|x| x == sue_to_find.samoyeds.unwrap())
                    && sue
                        .pomeranians
                        .is_none_or(|x| x == sue_to_find.pomeranians.unwrap())
                    && sue.akitas.is_none_or(|x| x == sue_to_find.akitas.unwrap())
                    && sue
                        .vizslas
                        .is_none_or(|x| x == sue_to_find.vizslas.unwrap())
                    && sue
                        .goldfish
                        .is_none_or(|x| x == sue_to_find.goldfish.unwrap())
                    && sue.trees.is_none_or(|x| x == sue_to_find.trees.unwrap())
                    && sue.cars.is_none_or(|x| x == sue_to_find.cars.unwrap())
                    && sue
                        .perfumes
                        .is_none_or(|x| x == sue_to_find.perfumes.unwrap())
            })
            .unwrap()
            .0
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
