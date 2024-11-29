use std::fmt::{Display, Formatter};

#[derive(Clone)]
pub enum Year {
    Year2023,
}

impl Display for Year {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let year = match self {
            Year::Year2023 => "2023",
        };

        write!(f, "{}", year)
    }
}
