pub mod day01;
pub mod day02;
pub mod day03;
pub mod day04;
pub mod day05;
pub mod day06;
pub mod day07;

pub trait Solution {
    fn part_one(&self, input: &str) -> String;
    fn part_two(&self, input: &str) -> String;
}

pub fn get_solutions() -> [Box<dyn Solution>; 7] {
    [
        Box::new(day01::Day01),
        Box::new(day02::Day02),
        Box::new(day03::Day03),
        Box::new(day04::Day04),
        Box::new(day05::Day05),
        Box::new(day06::Day06),
        Box::new(day07::Day07),
    ]
}