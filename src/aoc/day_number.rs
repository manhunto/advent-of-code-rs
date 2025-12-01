use std::fmt::{Display, Formatter};

#[derive(Clone, Copy, Debug)]
pub struct DayNumber {
    number: u8,
}

impl DayNumber {
    pub fn new(number: u8) -> Self {
        Self { number }
    }

    pub fn value(&self) -> u8 {
        self.number
    }
}

impl TryFrom<String> for DayNumber {
    type Error = String;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Ok(Self::new(value.trim_start_matches('0').parse().unwrap()))
    }
}

impl TryFrom<u8> for DayNumber {
    type Error = String;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        Ok(Self::new(value))
    }
}

impl From<DayNumber> for u8 {
    fn from(val: DayNumber) -> Self {
        val.number
    }
}

impl From<DayNumber> for u32 {
    fn from(day: DayNumber) -> Self {
        day.number as u32
    }
}

impl Display for DayNumber {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:0>2}", self.number)
    }
}

#[cfg(test)]
mod tests {
    use crate::aoc::day_number::DayNumber;

    #[test]
    fn to_string_test() {
        assert_eq!("01", DayNumber::new(1).to_string());
        assert_eq!("02", DayNumber::new(2).to_string());
        assert_eq!("10", DayNumber::new(10).to_string());
        assert_eq!("24", DayNumber::new(24).to_string());
    }

    #[test]
    fn try_from_string() {
        assert_eq!(1, DayNumber::try_from(String::from("01")).unwrap().number);
        assert_eq!(10, DayNumber::try_from(String::from("10")).unwrap().number);
        assert_eq!(24, DayNumber::try_from(String::from("24")).unwrap().number);
    }
}
