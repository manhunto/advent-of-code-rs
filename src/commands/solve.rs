use crate::aoc::client;
use crate::solutions::solution;
use crate::utils::day_number::DayNumber;
use crate::utils::file_system::{read_input, read_output};
use crate::utils::puzzle_part::PuzzlePart;
use crate::utils::year::Year;
use aoc_client::{AocResult, SubmissionOutcome};
use std::fmt::{Display, Formatter};
use std::time::{Duration, Instant};

pub fn solve(day_number: DayNumber, year: Year, submit_answer: Option<PuzzlePart>) {
    let solution = solution(day_number, year.clone());

    let input = match read_input(day_number.to_string().as_str(), year.clone()) {
        Ok(val) => val,
        Err(_) => panic!("Failed to read input. Download it first."), // todo better handle errors
    };

    let output = read_output(day_number.to_string().as_str(), year.clone());

    let expected: Vec<String> = output
        .unwrap_or(String::from(""))
        .lines()
        .map(|s| s.to_string())
        .collect();

    let expected_part_one = expected.first();
    let expected_part_two = expected.get(1);

    let solve_fn_part_one = || solution.part_one(&input);
    let result_part_one = run("one", &solve_fn_part_one, expected_part_one);
    println!("{}", result_part_one);

    let solve_fn_part_two = || solution.part_two(&input);
    let result_part_two = run("two", &solve_fn_part_two, expected_part_two);
    println!("{}", result_part_two);

    submit_answer_function(
        day_number,
        year,
        submit_answer,
        result_part_one,
        result_part_two,
    );
}

fn submit_answer_function(
    day_number: DayNumber,
    year: Year,
    submit_answer: Option<PuzzlePart>,
    result_part_one: SolutionResult,
    result_part_two: SolutionResult,
) -> bool {
    if let Some(part) = submit_answer {
        println!("=== Submitting answer ===");
        let solution_result = match part.clone() {
            PuzzlePart::PartOne => result_part_one,
            PuzzlePart::PartTwo => result_part_two,
        };

        if let Some(expected) = solution_result.expected {
            println!("There is already expected answer - {}", expected);
            println!("Submitting skipped");

            return true;
        }

        println!("Submitting answer..");
        let client = client(day_number, year);
        let outcome = client.submit_answer(part.clone(), solution_result.current);

        match outcome {
            Ok(o) => match o {
                SubmissionOutcome::Correct => println!("✅ Answer is correct"),
                SubmissionOutcome::Incorrect => println!("❌ Answer is incorrect. Try again"),
                SubmissionOutcome::Wait => println!("⏳ Another answer submitted tu early. Wait and submit again in a while"),
                SubmissionOutcome::WrongLevel => match part {
                    PuzzlePart::PartOne => println!("Wrong level. Maybe this part has been already solved?"),
                    PuzzlePart::PartTwo => println!("Wrong level. Maybe this part has already been solved or part one isn't solved?"),
                }
            },
            Err(err) => println!("There is an communication error with AoC server: {}", err),
        }
    };
    false
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
