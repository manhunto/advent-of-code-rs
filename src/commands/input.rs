use crate::utils::day_number::DayNumber;
use crate::utils::file_system::{read_input, write_input};
use crate::utils::year::Year;
use aoc_client::AocClient;

pub fn download_input(day_number: DayNumber, year: Year) {
    let input = read_input(day_number.to_string().as_str(), year.clone());

    match input {
        Ok(_) => println!("Input already exists."),
        Err(_) => {
            println!("Downloading...");
            let session = std::env::var("SESSION_COOKIE_ENV_VAR").unwrap();

            let client = AocClient::builder()
                .session_cookie(session)
                .unwrap()
                .year(year.clone() as i32)
                .unwrap()
                .day(u32::from(day_number))
                .unwrap()
                .build()
                .unwrap();

            let input = client.get_input().unwrap();

            write_input(&day_number.to_string(), year.clone(), &input).unwrap();

            println!("Input downloaded");
        }
    }
}
