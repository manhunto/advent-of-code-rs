use clap::builder::PossibleValue;
use clap::ValueEnum;
use std::fmt::{Display, Formatter};

#[derive(Clone, Debug)]
pub enum Year {
    Year2023 = 2023,
    Year2024 = 2024,
}

impl Display for Year {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let year = match self {
            Year::Year2023 => "2023",
            Year::Year2024 => "2024",
        };

        write!(f, "{}", year)
    }
}

impl ValueEnum for Year {
    fn value_variants<'a>() -> &'a [Self] {
        &[Self::Year2023, Self::Year2024]
    }

    fn to_possible_value(&self) -> Option<PossibleValue> {
        Some(match self {
            Self::Year2023 => PossibleValue::new("2023"),
            Self::Year2024 => PossibleValue::new("2024"),
        })
    }
}
