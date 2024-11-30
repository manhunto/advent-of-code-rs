use crate::utils::day_number::DayNumber;
use crate::solutions::year2023::day19;
use crate::utils::year::Year;
use year2023::{
    day01, day02, day03, day04, day05, day06, day07, day08, day09, day10, day11, day12, day13,
    day14, day15, day16, day17, day18, day20, day21, day22, day23, day24, day25,
};

mod year2023;

pub trait Solution {
    fn part_one(&self, input: &str) -> String;
    fn part_two(&self, input: &str) -> String;
}

pub fn solution(day: &DayNumber, year: Year) -> Box<dyn Solution> {
    let i: u8 = (*day).into();

    match year {
        Year::Year2023 => match i {
            1 => Box::new(day01::Day01),
            2 => Box::new(day02::Day02),
            3 => Box::new(day03::Day03),
            4 => Box::new(day04::Day04),
            5 => Box::new(day05::Day05),
            6 => Box::new(day06::Day06),
            7 => Box::new(day07::Day07),
            8 => Box::new(day08::Day08),
            9 => Box::new(day09::Day09),
            10 => Box::new(day10::Day10),
            11 => Box::new(day11::Day11),
            12 => Box::new(day12::Day12),
            13 => Box::new(day13::Day13),
            14 => Box::new(day14::Day14),
            15 => Box::new(day15::Day15),
            16 => Box::new(day16::Day16),
            17 => Box::new(day17::Day17),
            18 => Box::new(day18::Day18),
            19 => Box::new(day19::Day19),
            20 => Box::new(day20::Day20),
            21 => Box::new(day21::Day21),
            22 => Box::new(day22::Day22),
            23 => Box::new(day23::Day23),
            24 => Box::new(day24::Day24),
            25 => Box::new(day25::Day25),
            _ => panic!("Day not exist"),
        },
    }
}
