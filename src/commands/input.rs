use crate::aoc::client;
use crate::aoc::day_number::DayNumber;
use crate::aoc::file_system::{read_input, write_input};
use crate::aoc::year::Year;

pub fn download_input(day_number: DayNumber, year: Year) {
    let input = read_input(day_number.to_string().as_str(), year.clone());

    match input {
        Ok(_) => println!("Input already exists."),
        Err(_) => {
            println!("Downloading...");
            let client = client(day_number, year.clone());

            let input = client.get_input().unwrap();

            write_input(&day_number.to_string(), year.clone(), &input).unwrap();

            println!("Input downloaded");
        }
    }
}
