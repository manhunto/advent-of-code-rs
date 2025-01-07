use crate::aoc::day_number::DayNumber;
use crate::aoc::year::Year;

mod year2023;
mod year2024;

pub trait Solution {
    fn part_one(&self, input: &str) -> String;
    fn part_two(&self, input: &str) -> String;
}

pub fn solution(day: DayNumber, year: Year) -> Box<dyn Solution> {
    let i: u8 = day.into();

    match year {
        Year::Year2024 => match i {
            1 => Box::new(year2024::day01::Day01),
            2 => Box::new(year2024::day02::Day02),
            3 => Box::new(year2024::day03::Day03),
            4 => Box::new(year2024::day04::Day04),
            5 => Box::new(year2024::day05::Day05),
            6 => Box::new(year2024::day06::Day06),
            7 => Box::new(year2024::day07::Day07),
            8 => Box::new(year2024::day08::Day08),
            9 => Box::new(year2024::day09::Day09),
            10 => Box::new(year2024::day10::Day10),
            11 => Box::new(year2024::day11::Day11),
            12 => Box::new(year2024::day12::Day12),
            13 => Box::new(year2024::day13::Day13),
            14 => Box::new(year2024::day14::Day14::default()),
            15 => Box::new(year2024::day15::Day15),
            16 => Box::new(year2024::day16::Day16),
            17 => Box::new(year2024::day17::Day17),
            18 => Box::new(year2024::day18::Day18::default()),
            19 => Box::new(year2024::day19::Day19),
            20 => Box::new(year2024::day20::Day20),
            21 => Box::new(year2024::day21::Day21),
            22 => Box::new(year2024::day22::Day22),
            23 => Box::new(year2024::day23::Day23),
            24 => Box::new(year2024::day24::Day24),
            _ => panic!("Day not exist"),
        },
        Year::Year2023 => match i {
            1 => Box::new(year2023::day01::Day01),
            2 => Box::new(year2023::day02::Day02),
            3 => Box::new(year2023::day03::Day03),
            4 => Box::new(year2023::day04::Day04),
            5 => Box::new(year2023::day05::Day05),
            6 => Box::new(year2023::day06::Day06),
            7 => Box::new(year2023::day07::Day07),
            8 => Box::new(year2023::day08::Day08),
            9 => Box::new(year2023::day09::Day09),
            10 => Box::new(year2023::day10::Day10),
            11 => Box::new(year2023::day11::Day11),
            12 => Box::new(year2023::day12::Day12),
            13 => Box::new(year2023::day13::Day13),
            14 => Box::new(year2023::day14::Day14),
            15 => Box::new(year2023::day15::Day15),
            16 => Box::new(year2023::day16::Day16),
            17 => Box::new(year2023::day17::Day17),
            18 => Box::new(year2023::day18::Day18),
            19 => Box::new(year2023::day19::Day19),
            20 => Box::new(year2023::day20::Day20),
            21 => Box::new(year2023::day21::Day21),
            22 => Box::new(year2023::day22::Day22),
            23 => Box::new(year2023::day23::Day23),
            24 => Box::new(year2023::day24::Day24),
            25 => Box::new(year2023::day25::Day25),
            _ => panic!("Day not exist"),
        },
    }
}
