use std::env;
use std::fmt::{Display, Formatter};
use std::time::{Duration, Instant};
use crate::day::DayNumber;
use crate::solutions::{get_solutions};

mod file_system;
mod solutions;
mod day;
mod range;
mod chain_pattern_finder;
mod math;
mod point;
mod direction;
mod grid;
mod pair_generator;

mod shoelace_formula;
mod utils;

fn main() {
    let args: Vec<String> = env::args().collect();
    let day_number: DayNumber = DayNumber::new(*&args.get(1).expect("Add day number").parse().expect("Invalid format"));

    let solutions = get_solutions();
    let solution = &solutions[(day_number.as_u8() - 1) as usize];

    let input = file_system::read_input(day_number.as_string().as_str());
    let output = file_system::read_output(day_number.as_string().as_str());

    let lines: Vec<String> = output.unwrap_or(String::from("")).lines().map(|s| s.to_string()).collect();
    let expected_part_one = lines.get(0);
    let expected_part_two = lines.get(1);

    let start_part_one = Instant::now();
    let part_one: String = solution.part_one(&input.as_str());
    let duration_part_one = start_part_one.elapsed();

    let part_one_result = Result {
        expected: expected_part_one,
        current: part_one.clone(),
        elapsed: duration_part_one,
    };

    let start_part_two = Instant::now();
    let part_two: String = solution.part_two(&input.as_str());
    let duration_part_two = start_part_two.elapsed();

    let part_two_result = Result {
        expected: expected_part_two,
        current: part_two.clone(),
        elapsed: duration_part_two,
    };

    println!("{}", part_one_result);
    println!("{}", part_two_result);
}

struct Result<'a> {
    expected: Option<&'a String>,
    current: String,
    elapsed: Duration,
}

impl Display for Result<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let result = match self.expected {
            None => "❔",
            Some(value) => if value == &self.current { "✅" } else { "❌" }
        };

        write!(f, "Part one: {} ({:?}) {}", self.current, self.elapsed, result)
    }
}
