use crate::aoc::client;
use crate::aoc::file_system::{read_input, write_input};
use crate::aoc::puzzle_day::PuzzleDay;

pub fn download_input(puzzle_day: PuzzleDay) {
    let day_number = puzzle_day.day_number();
    let year = puzzle_day.year();

    let input = read_input(day_number.to_string().as_str(), year);

    match input {
        Ok(_) => println!("Input already exists."),
        Err(_) => {
            println!("Downloading...");
            let client = client(day_number, year);

            let input = client.get_input().unwrap();

            write_input(&day_number.to_string(), year, &input).unwrap();

            println!("Input downloaded");
        }
    }
}
