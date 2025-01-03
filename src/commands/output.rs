use crate::aoc::client;
use crate::aoc::day_number::DayNumber;
use crate::aoc::expected_result::ExpectedResult;
use crate::aoc::file_system::{read_output, write_output};
use crate::aoc::year::Year;
use regex::Regex;

pub fn download_output(day_number: Option<DayNumber>, year: Year, force: bool) {
    if let Some(day_number) = day_number {
        download_day(day_number, year, force)
    } else {
        for day in DayNumber::all() {
            download_day(day, year.clone(), force);
        }
    }
}

fn download_day(day_number: DayNumber, year: Year, force: bool) {
    println!("=== Day {} in {} ===", day_number, year);

    let current = read_output(day_number.to_string().as_str(), year.clone());
    let client = client(day_number, year.clone());
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
        write_output(
            day_number.to_string().as_str(),
            year.clone(),
            expected_result,
        )
        .unwrap();
        println!("Expected result persisted");
    } else {
        println!("Skipped. Already exist. Use --force to overwrite.");
    }
}
