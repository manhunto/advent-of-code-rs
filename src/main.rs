use crate::day::DayNumber;
use crate::solutions::get_solutions;
use std::env;
use std::fmt::{Display, Formatter};
use std::time::{Duration, Instant};

mod chain_pattern_finder;
mod day;
mod direction;
mod file_system;
mod grid;
mod math;
mod pair_generator;
mod point;
mod range;
mod shoelace_formula;
mod solutions;
mod utils;

fn main() {
    let args: Vec<String> = env::args().collect();
    let day_number: DayNumber = DayNumber::new(
        args.get(1)
            .expect("Add day number")
            .parse()
            .expect("Invalid format"),
    );

    let solutions = get_solutions();
    let solution = &solutions[(day_number.as_u8() - 1) as usize];

    let input = file_system::read_input(day_number.as_string().as_str());
    let output = file_system::read_output(day_number.as_string().as_str());

    let expected: Vec<String> = output
        .unwrap_or(String::from(""))
        .lines()
        .map(|s| s.to_string())
        .collect();
    let expected_part_one = expected.first();
    let expected_part_two = expected.get(1);

    println!(
        "{}",
        run("one", &|| solution.part_one(&input), expected_part_one)
    );
    println!(
        "{}",
        run("two", &|| solution.part_two(&input), expected_part_two)
    );
}

fn run<'a>(
    part: &str,
    solve_fn: &'a dyn Fn() -> String,
    expected: Option<&'a String>,
) -> Result<'a> {
    let start = Instant::now();
    let current: String = solve_fn();
    let elapsed = start.elapsed();

    Result {
        part: part.to_string(),
        expected,
        current,
        elapsed,
    }
}

struct Result<'a> {
    part: String,
    expected: Option<&'a String>,
    current: String,
    elapsed: Duration,
}

impl Display for Result<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let result = match self.expected {
            None => "❔",
            Some(value) => {
                if value == &self.current {
                    "✅"
                } else {
                    "❌"
                }
            }
        };
        let elapsed_in_ms = self.elapsed.as_nanos() as f64 / 1000.0 / 1000.0;

        write!(
            f,
            "Part {}: {} ({:.3}ms) {}",
            self.part, self.current, elapsed_in_ms, result
        )
    }
}
