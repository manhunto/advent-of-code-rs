use crate::aoc::day_number::DayNumber;
use crate::aoc::year::Year;
use crate::aoc::year::Year::Year2025;
use std::ops::RangeInclusive;

pub struct PuzzleDay {
    day_number: DayNumber,
    year: Year,
}

impl PuzzleDay {
    pub fn new(day_number: DayNumber, year: Year) -> Result<Self, String> {
        let day_value = day_number.value();

        let range = Self::day_range_for_year(year);

        if !range.contains(&day_value) {
            return Err(format!(
                "Day number must be between {} and {}. But was: {}",
                range.start(),
                range.end(),
                day_value
            ));
        }

        Ok(PuzzleDay { day_number, year })
    }

    pub fn day_number(&self) -> DayNumber {
        self.day_number
    }

    pub fn year(&self) -> Year {
        self.year
    }

    pub fn all_for_year(year: Year) -> Vec<PuzzleDay> {
        Self::day_range_for_year(year)
            .map(|day| PuzzleDay::new(DayNumber::try_from(day).unwrap(), year).unwrap())
            .collect()
    }

    fn day_range_for_year(year: Year) -> RangeInclusive<u8> {
        if year < Year2025 {
            return 1..=25;
        }

        1..=12
    }
}

#[cfg(test)]
mod tests {
    use crate::aoc::day_number::DayNumber;
    use crate::aoc::puzzle_day::PuzzleDay;
    use crate::aoc::year::Year::{Year2023, Year2024, Year2025};

    #[test]
    fn construct_test() {
        assert!(PuzzleDay::new(DayNumber::new(1), Year2024).is_ok());
        assert!(PuzzleDay::new(DayNumber::new(2), Year2024).is_ok());
        assert!(PuzzleDay::new(DayNumber::new(25), Year2024).is_ok());
        assert!(PuzzleDay::new(DayNumber::new(25), Year2023).is_ok());
        assert!(PuzzleDay::new(DayNumber::new(26), Year2024).is_err());

        assert!(PuzzleDay::new(DayNumber::new(12), Year2025).is_ok());
        assert!(PuzzleDay::new(DayNumber::new(13), Year2025).is_err());
    }
}
