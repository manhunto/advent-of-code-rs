pub mod day01;
pub mod day02;
pub mod day03;

pub trait Solution {
    fn part_one(&self, input: &str) -> String;
    fn part_two(&self, input: &str) -> String;
}

pub fn get_solutions() -> [Box<dyn Solution>; 3] {
    [
        Box::new(day01::Day01),
        Box::new(day02::Day02),
        Box::new(day03::Day03),
    ]
}