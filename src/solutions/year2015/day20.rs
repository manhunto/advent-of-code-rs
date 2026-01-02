use crate::solutions::Solution;

pub struct Day20;

impl Solution for Day20 {
    fn part_one(&self, input: &str) -> String {
        self.solve_houses(input, 10, usize::MAX)
    }

    fn part_two(&self, input: &str) -> String {
        self.solve_houses(input, 11, 50)
    }
}

impl Day20 {
    fn solve_houses(&self, input: &str, presents_per_elf: usize, max_visits: usize) -> String {
        let target: usize = input.trim().parse().expect("Invalid input number");
        let limit = target / presents_per_elf;
        let mut houses = vec![0; limit];

        for elf in 1..limit {
            let presents = elf * presents_per_elf;
            for house in (elf..limit).step_by(elf).take(max_visits) {
                houses[house] += presents;
            }
        }

        houses
            .into_iter()
            .position(|presents| presents >= target)
            .unwrap_or(0)
            .to_string()
    }
}
