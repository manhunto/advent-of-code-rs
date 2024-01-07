pub struct DayNumber {
    number: u8,
}

impl DayNumber {
    pub fn new(number: u8) -> Self {
        Self { number }
    }

    pub fn as_string(&self) -> String {
        format!("{:0>2}", self.number)
    }

    pub fn as_u8(&self) -> u8 {
        self.number
    }
}

#[cfg(test)]
mod tests {
    use crate::day::DayNumber;

    #[test]
    fn as_string_test() {
        assert_eq!("01", DayNumber::new(1).as_string());
        assert_eq!("02", DayNumber::new(2).as_string());
        assert_eq!("10", DayNumber::new(10).as_string());
        assert_eq!("24", DayNumber::new(24).as_string());
    }
}
