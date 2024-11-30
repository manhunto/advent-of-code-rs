use crate::commands::input::download_input;
use crate::commands::solve::{solve};
use crate::utils::year::Year;
use clap::{Parser, Subcommand};
use dotenv::dotenv;
use utils::day_number::DayNumber;
use utils::year::Year::Year2023;
use crate::utils::puzzle_part::PuzzlePart;

mod aoc;
mod commands;
mod solutions;
mod utils;

const DEFAULT_CMD: Command = Command::Solve {
    submit_answer: None,
};

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Cli {
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
    /// Run solver for given puzzle
    #[clap(short_flag = 's')]
    Solve {
        /// If provided it sends answer to AoC server for given part
        #[clap(short = 'a')]
        submit_answer: Option<PuzzlePart>,
    },
    /// Downloads and saves input for given puzzle
    #[clap(short_flag = 'i')]
    Input,
}

fn parse_day(s: &str) -> Result<u8, String> {
    match s.parse::<u8>() {
        Ok(n) if (1..=25).contains(&n) => Ok(n),
        Ok(_) => Err("The number must be between 1 and 25.".to_string()),
        Err(_) => Err("Invalid number provided.".to_string()),
    }
}

fn main() {
    dotenv().ok();

    let cli = Cli::parse();
    let command = cli.command.unwrap_or(DEFAULT_CMD);
    let day = cli.day.unwrap_or(1);
    let day_number: DayNumber = DayNumber::try_from(day.to_string()).unwrap();

    let year = cli.year.unwrap_or(Year2023);

    println!("=== Day {} in {} ===", day_number, year);

    match command {
        Command::Solve { submit_answer } => solve(day_number, year, submit_answer),
        Command::Input => download_input(day_number, year),
    }
}
