use crate::solutions::Solution;

pub struct Day20;

const ELF_PRESENTS_PART_ONE: usize = 10;
const ELF_PRESENTS_PART_TWO: usize = 11;

impl Solution for Day20 {
    fn part_one(&self, input: &str) -> String {
        let target: usize = input.trim().parse().unwrap();
        let limit = target / ELF_PRESENTS_PART_ONE + 1;
        let mut houses = vec![0; limit];

        for elf in 1..limit {
            let presents = elf * ELF_PRESENTS_PART_ONE;

            for house in (elf..limit).step_by(elf) {
                houses[house] += presents;
            }
        }

        houses
            .iter()
            .position(|&presents| presents >= target)
            .unwrap_or(0)
            .to_string()
    }

    fn part_two(&self, input: &str) -> String {
        let target: usize = input.trim().parse().unwrap();
        let limit = target / ELF_PRESENTS_PART_TWO + 1;
        let mut houses = vec![0; limit];

        for elf in 1..limit {
            let presents = elf * ELF_PRESENTS_PART_TWO;

            for house in (elf..limit).step_by(elf).take(50) {
                houses[house] += presents;
            }
        }

        houses
            .iter()
            .position(|&presents| presents >= target)
            .unwrap_or(0)
            .to_string()
    }
}
