use aoc_client::AocError;
use clap::builder::PossibleValue;
use clap::ValueEnum;
use std::fmt::{Display, Formatter};

#[derive(Clone, Debug)]
pub enum PuzzlePart {
    PartOne,
    PartTwo,
}

impl ValueEnum for PuzzlePart {
    fn value_variants<'a>() -> &'a [Self] {
        &[Self::PartOne, Self::PartTwo]
    }

    fn to_possible_value(&self) -> Option<PossibleValue> {
        Some(match self {
            Self::PartOne => PossibleValue::new("1"),
            Self::PartTwo => PossibleValue::new("2"),
        })
    }
}

impl TryInto<aoc_client::PuzzlePart> for PuzzlePart {
    type Error = AocError;

    fn try_into(self) -> Result<aoc_client::PuzzlePart, Self::Error> {
        Ok(match self {
            Self::PartOne => aoc_client::PuzzlePart::PartOne,
            Self::PartTwo => aoc_client::PuzzlePart::PartTwo,
        })
    }
}

impl Display for PuzzlePart {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let value = match self {
            PuzzlePart::PartOne => "Part one",
            PuzzlePart::PartTwo => "Part two",
        };

        write!(f, "{}", value)
    }
}
