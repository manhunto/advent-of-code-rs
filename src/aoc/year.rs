use clap::builder::PossibleValue;
use clap::ValueEnum;
use std::cmp::Ordering;
use std::fmt::{Display, Formatter};

#[derive(Clone, Debug, Eq, PartialEq, Copy)]
pub enum Year {
    Year2023 = 2023,
    Year2024 = 2024,
    Year2025 = 2025,
}

impl Display for Year {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let year = match self {
            Self::Year2023 => "2023",
            Self::Year2024 => "2024",
            Self::Year2025 => "2025",
        };

        write!(f, "{}", year)
    }
}

impl Ord for Year {
    fn cmp(&self, other: &Self) -> Ordering {
        (*self as u32).cmp(&(*other as u32))
    }
}

impl PartialOrd for Year {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
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
            Self::Year2025 => PossibleValue::new("2025"),
        })
    }
}
