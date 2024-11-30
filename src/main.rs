use crate::commands::input::download_input;
use crate::commands::output::download_output;
use crate::commands::solve::solve;
use aoc::day_number::DayNumber;
use aoc::puzzle_part::PuzzlePart;
use aoc::year::Year;
use aoc::year::Year::Year2023;
use clap::{Parser, Subcommand};
use dotenv::dotenv;

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
    /// Downloads and saves output for given or if day wasn't provided it fetches for whole year
    #[clap(short_flag = 'o')]
    Output {
        #[arg(short, long, help = "Force download even if exists")]
        force: bool,
    },
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
    let day_option = cli.day;
    let day_number_option = day_option.map(|d| DayNumber::try_from(d).unwrap());
    let day_number: DayNumber = DayNumber::try_from(day_option.unwrap_or(1).to_string()).unwrap();

    let year = cli.year.unwrap_or(Year2023);

    println!("=== Day {} in {} ===", day_number, year);

    match command {
        Command::Solve { submit_answer } => solve(day_number, year, submit_answer),
        Command::Input => download_input(day_number, year),
        Command::Output { force } => download_output(day_number_option, year, force),
    }
}
