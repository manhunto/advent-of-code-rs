use std::env;
use std::time::Instant;
use crate::day::DayNumber;
use crate::solutions::{get_solutions};

mod file_system;
mod solutions;
mod day;
mod range;
mod infinite_iterator;
mod chain_pattern_finder;
mod math;
mod point;
mod direction;
mod grid;

mod shoelace_formula;

fn main() {
    let args: Vec<String> = env::args().collect();
    let day_number: DayNumber = DayNumber::new(*&args.get(1).expect("Add day number").parse().expect("Invalid format"));

    let solutions = get_solutions();
    let solution = &solutions[(day_number.as_u8() - 1) as usize];

    let input = file_system::read_input(day_number.as_string().as_str());

    let start_part_one = Instant::now();
    let part_one: String = solution.part_one(&input.as_str());
    let duration_part_one = start_part_one.elapsed();

    let start_part_two = Instant::now();
    let part_two: String = solution.part_two(&input.as_str());
    let duration_part_two = start_part_two.elapsed();

    println!("Part one: {} ({:?})", part_one, duration_part_one);
    println!("Part two: {} ({:?})", part_two, duration_part_two);
}
