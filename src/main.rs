use crate::solutions::solution;
use crate::utils::year::Year;
use clap::{Parser, Subcommand};
use std::fmt::{Display, Formatter};
use std::time::{Duration, Instant};
use utils::day_number::DayNumber;
use utils::file_system::{read_input, read_output};
use utils::year::Year::Year2023;

mod solutions;
mod utils;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    #[command(subcommand)]
    command: Option<Command>,
    #[clap(short, long)]
    #[arg(value_parser = clap::builder::ValueParser::new(parse_day), help = "A number between 1 and 25")]
    day: Option<u8>,
    #[clap(short, long)]
    year: Option<Year>,
}

#[derive(Subcommand, Debug)]
enum Command {
    #[clap(short_flag = 's')]
    Solve,
}

fn parse_day(s: &str) -> Result<u8, String> {
    match s.parse::<u8>() {
        Ok(n) if (1..=25).contains(&n) => Ok(n),
        Ok(_) => Err("The number must be between 1 and 25.".to_string()),
        Err(_) => Err("Invalid number provided.".to_string()),
    }
}

fn main() {
    let cli = Args::parse();
    let command = cli.command.unwrap_or(Command::Solve);
    let day = cli.day.unwrap_or(1);
    let day_number: DayNumber = DayNumber::try_from(day.to_string()).unwrap();

    let year = cli.year.unwrap_or(Year2023);

    println!("=== Day {} in {} ===", day_number, year);

    match command {
        Command::Solve => solve(&day_number, year),
    }
}

fn solve(day_number: &DayNumber, year: Year) {
    let solution = solution(&day_number, year.clone());

    let input = read_input(day_number.to_string().as_str(), year.clone());
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
