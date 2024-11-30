use crate::aoc::puzzle_part::PuzzlePart;
use itertools::Itertools;

#[derive(Debug, Clone)]
pub struct ExpectedResult {
    part_one_result: Option<String>,
    part_two_result: Option<String>,
}

impl From<String> for ExpectedResult {
    fn from(value: String) -> Self {
        let expected: Vec<String> = value.lines().map(|s| s.to_string()).collect();

        if expected.len() > 2 {
            panic!("Expected max 2 lines, got {}", expected.len())
        }

        let part_one_result = expected.first().map(|s| s.to_string());
        let part_two_result = expected.get(1).map(|s| s.to_string());

        Self {
            part_one_result,
            part_two_result,
        }
    }
}

impl From<Vec<String>> for ExpectedResult {
    fn from(value: Vec<String>) -> Self {
        ExpectedResult::from(value.join("\n"))
    }
}

impl From<ExpectedResult> for String {
    fn from(value: ExpectedResult) -> Self {
        let vec = [value.part_one_result, value.part_two_result];

        vec.iter().filter_map(|s| s.clone()).join("\n")
    }
}

impl ExpectedResult {
    pub fn get_for_part(&self, part: PuzzlePart) -> Option<String> {
        match part {
            PuzzlePart::PartOne => self.part_one_result.clone(),
            PuzzlePart::PartTwo => self.part_two_result.clone(),
        }
    }

    pub fn is_empty(&self) -> bool {
        self.part_one_result.is_none() && self.part_two_result.is_none()
    }
}
