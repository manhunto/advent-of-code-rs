use std::fmt::{self, Binary as BinaryFmt, Display, Formatter};
use std::str::FromStr;

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Binary(usize);

impl Binary {
    /// Creates a new Binary from a usize value
    pub const fn new(value: usize) -> Self {
        Self(value)
    }

    /// Returns the inner usize value
    pub const fn get(self) -> usize {
        self.0
    }
}

impl Display for Binary {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{:b}", self.0)
    }
}

impl BinaryFmt for Binary {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        BinaryFmt::fmt(&self.0, f)
    }
}

impl FromStr for Binary {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.is_empty() {
            return Err("cannot parse empty string as binary".to_string());
        }

        if !s.chars().all(|b| b == '0' || b == '1') {
            return Err(format!("'{}' is not a valid binary number", s));
        }

        usize::from_str_radix(s, 2)
            .map(Self)
            .map_err(|e| format!("failed to parse binary: {}", e))
    }
}

impl From<usize> for Binary {
    fn from(value: usize) -> Self {
        Self(value)
    }
}

impl From<Binary> for usize {
    fn from(value: Binary) -> Self {
        value.0
    }
}
