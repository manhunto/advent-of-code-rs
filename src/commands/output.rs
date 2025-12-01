use crate::aoc::client;
use crate::aoc::day_number::DayNumber;
use crate::aoc::expected_result::ExpectedResult;
use crate::aoc::file_system::{read_output, write_output};
use crate::aoc::puzzle_day::PuzzleDay;
use crate::aoc::year::Year;
use regex::Regex;

pub fn download_output(day_number: Option<DayNumber>, year: Year, force: bool) {
    if let Some(day_number) = day_number {
        let puzzle_day = PuzzleDay::new(day_number, year).unwrap();

        download_day(puzzle_day, force)
    } else {
        for puzzle_day in PuzzleDay::all_for_year(year) {
            download_day(puzzle_day, force);
        }
    }
}

fn download_day(puzzle_day: PuzzleDay, force: bool) {
    let day_number = puzzle_day.day_number();
    let year = puzzle_day.year();

    println!("=== Day {} in {} ===", day_number, year);

    let current = read_output(day_number.to_string().as_str(), year);
    let client = client(day_number, year);
    let content = client.get_puzzle_html().unwrap();
    let re = Regex::new(r"Your puzzle answer was <code>(.{1,100})</code>").unwrap();
    let matches: Vec<String> = re
        .captures_iter(&content)
        .map(|c| c[1].to_string())
        .collect();
    let expected_result = ExpectedResult::from(matches);
    if expected_result.is_empty() {
        println!("Skipped. Don't want to persist empty results");

        return;
    }

    if current.is_empty() || current != expected_result || force {
        write_output(day_number.to_string().as_str(), year, expected_result).unwrap();
        println!("Expected result persisted");
    } else {
        println!("Skipped. Already exist. Use --force to overwrite.");
    }
}
