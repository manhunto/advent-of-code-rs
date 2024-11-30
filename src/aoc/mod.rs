pub mod day_number;
pub mod expected_result;
pub mod file_system;
pub mod puzzle_part;
pub mod year;

use aoc_client::AocClient;
use day_number::DayNumber;
use year::Year;

pub fn client(day_number: DayNumber, year: Year) -> AocClient {
    let session = std::env::var("SESSION_COOKIE_ENV_VAR").unwrap();

    AocClient::builder()
        .session_cookie(session)
        .unwrap()
        .year(year.clone() as i32)
        .unwrap()
        .day(u32::from(day_number))
        .unwrap()
        .build()
        .unwrap()
}
