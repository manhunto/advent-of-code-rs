use crate::solutions::solution;
use crate::utils::day_number::DayNumber;
use crate::utils::file_system::{read_input, read_output};
use crate::utils::year::Year;
use std::fmt::{Display, Formatter};
use std::time::{Duration, Instant};

pub fn solve(day_number: &DayNumber, year: Year) {
    let solution = solution(*day_number, year.clone());

    let input = match read_input(day_number.to_string().as_str(), year.clone()) {
        Ok(val) => val,
        Err(_) => panic!("Failed to read input. Download it first."), // todo better handle errors
    };

    let output = read_output(day_number.to_string().as_str(), year);

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
) -> SolutionResult<'a> {
    let start = Instant::now();
    let current: String = solve_fn();
    let elapsed = start.elapsed();

    SolutionResult {
        part: part.to_string(),
        expected,
        current,
        elapsed,
    }
}

struct SolutionResult<'a> {
    part: String,
    expected: Option<&'a String>,
    current: String,
    elapsed: Duration,
}

impl Display for SolutionResult<'_> {
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
