use crate::utils::day_number::DayNumber;
use crate::utils::year::Year;
use aoc_client::AocClient;

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
