pub mod day01;
pub mod day02;

pub trait Solution {
    fn part_one(&self, input: &str) -> String;
    fn part_two(&self, input: &str) -> String;
}

pub fn get_solutions() -> [Box<dyn Solution>; 2] {
    [
        Box::new(day01::Day01),
        Box::new(day02::Day02),
    ]
}