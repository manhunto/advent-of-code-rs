pub mod day01;
pub mod day02;
pub mod day03;
pub mod day04;
pub mod day05;
pub mod day06;
pub mod day07;
pub mod day08;
pub mod day09;
pub mod day10;
pub mod day11;
pub mod day12;
pub mod day13;
pub mod day14;
pub mod day15;
pub mod day16;
pub mod day17;
pub mod day18;
pub mod day19;
pub mod day20;
pub mod day21;
pub mod day22;
pub mod day23;

pub trait Solution {
    fn part_one(&self, input: &str) -> String;
    fn part_two(&self, input: &str) -> String;
}

pub fn get_solutions() -> [Box<dyn Solution>; 23] {
    [
        Box::new(day01::Day01),
        Box::new(day02::Day02),
        Box::new(day03::Day03),
        Box::new(day04::Day04),
        Box::new(day05::Day05),
        Box::new(day06::Day06),
        Box::new(day07::Day07),
        Box::new(day08::Day08),
        Box::new(day09::Day09),
        Box::new(day10::Day10),
        Box::new(day11::Day11),
        Box::new(day12::Day12),
        Box::new(day13::Day13),
        Box::new(day14::Day14),
        Box::new(day15::Day15),
        Box::new(day16::Day16),
        Box::new(day17::Day17),
        Box::new(day18::Day18),
        Box::new(day19::Day19),
        Box::new(day20::Day20),
        Box::new(day21::Day21),
        Box::new(day22::Day22),
        Box::new(day23::Day23),
    ]
}
