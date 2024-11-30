use aoc_client::AocError;
use clap::builder::PossibleValue;
use clap::ValueEnum;

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
            Self::PartTwo => aoc_client::PuzzlePart::PartTwo
        })
    }
}