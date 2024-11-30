use crate::aoc::client;
use crate::aoc::day_number::DayNumber;
use crate::aoc::expected_result::ExpectedResult;
use crate::aoc::file_system::{read_output, write_output};
use crate::aoc::year::Year;
use regex::Regex;

pub fn download_output(day_number: Option<DayNumber>, year: Year, force: bool) {
    let re = Regex::new(r"Your puzzle answer was <code>(.{1,30})</code>").unwrap();

    if let Some(day_number) = day_number {
        let current = read_output(day_number.to_string().as_str(), year.clone());

        if current.is_empty() || force {
            let client = client(day_number, year.clone());
            let content = client.get_puzzle_html().unwrap();

            let vec: Vec<String> = re
                .captures_iter(&content)
                .map(|c| c[1].to_string())
                .collect();
            let expected_result = ExpectedResult::from(vec);
            if expected_result.is_empty() {
                println!("Skipped. Don't want to persist empty results");

                return;
            }

            write_output(
                day_number.to_string().as_str(),
                year.clone(),
                expected_result,
            )
            .unwrap();
            println!("Expected result persisted for {}/{}", day_number, year);
        } else {
            println!("Skipped. Already exist. Use --force to overwrite.");
        }
    } else {
        todo!("Download whole year");
    }
}
