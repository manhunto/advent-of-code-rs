use crate::aoc::client;
use crate::aoc::day_number::DayNumber;
use crate::aoc::expected_result::ExpectedResult;
use crate::aoc::file_system::{read_input, read_output};
use crate::aoc::puzzle_day::PuzzleDay;
use crate::aoc::puzzle_part::PuzzlePart;
use crate::aoc::year::Year;
use crate::solutions::solution;
use aoc_client::SubmissionOutcome;
use std::fmt::{Display, Formatter};
use std::time::{Duration, Instant};

pub fn solve(puzzle_day: PuzzleDay, submit_answer: Option<PuzzlePart>) {
    let day_number = puzzle_day.day_number();
    let year = puzzle_day.year();

    let solution = solution(puzzle_day);

    let input = match read_input(day_number.to_string().as_str(), year) {
        Ok(val) => val,
        Err(_) => panic!("Failed to read input. Download it first."), // todo better handle errors
    };

    let expected = read_output(day_number.to_string().as_str(), year);

    let solve_fn_part_one = || solution.part_one(&input);
    let result_part_one = run(PuzzlePart::PartOne, &solve_fn_part_one, expected.clone());
    println!("{}", result_part_one);

    let solve_fn_part_two = || solution.part_two(&input);
    let result_part_two = run(PuzzlePart::PartTwo, &solve_fn_part_two, expected);
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

fn run(
    part: PuzzlePart,
    solve_fn: &dyn Fn() -> String,
    expected: ExpectedResult,
) -> SolutionResult {
    let start = Instant::now();
    let current: String = solve_fn();
    let elapsed = start.elapsed();

    SolutionResult {
        part: part.clone(),
        expected: expected.get_for_part(part),
        current,
        elapsed,
    }
}

struct SolutionResult {
    part: PuzzlePart,
    expected: Option<String>,
    current: String,
    elapsed: Duration,
}

impl Display for SolutionResult {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let result = match self.expected.clone() {
            None => "❔",
            Some(value) => {
                if value == self.current {
                    "✅"
                } else {
                    "❌"
                }
            }
        };
        let elapsed_in_ms = self.elapsed.as_nanos() as f64 / 1000.0 / 1000.0;

        write!(
            f,
            "{}: {} ({:.3}ms) {}",
            self.part, self.current, elapsed_in_ms, result
        )
    }
}
