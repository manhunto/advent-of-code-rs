use crate::aoc::expected_result::ExpectedResult;
use crate::aoc::year::Year;
use std::fmt::{Display, Formatter};
use std::fs;
use std::fs::read_to_string;

enum ResourceType {
    Inputs,
    Outputs,
    #[cfg(test)]
    Examples,
}

impl Display for ResourceType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let resource_type = match self {
            ResourceType::Inputs => "inputs",
            ResourceType::Outputs => "outputs",
            #[cfg(test)]
            ResourceType::Examples => "examples",
        };

        write!(f, "{}", resource_type)
    }
}

pub fn write_input(day: &str, year: Year, data: &str) -> std::io::Result<()> {
    let file_path = build_path(ResourceType::Inputs, day, year);

    fs::write(file_path, data)
}

pub fn read_input(day: &str, year: Year) -> std::io::Result<String> {
    read(ResourceType::Inputs, day, year)
}

pub fn write_output(day: &str, year: Year, expected_result: ExpectedResult) -> std::io::Result<()> {
    let file_path = build_path(ResourceType::Outputs, day, year);
    let data: String = expected_result.into();

    fs::write(file_path, data)
}

pub fn read_output(day: &str, year: Year) -> ExpectedResult {
    let content = read(ResourceType::Outputs, day, year);

    ExpectedResult::from(content.unwrap_or_default())
}

#[cfg(test)]
pub fn read_example(day: &str, year: Year) -> String {
    read(ResourceType::Examples, day, year).unwrap()
}

fn read(resource_type: ResourceType, day: &str, year: Year) -> std::io::Result<String> {
    let file_path = build_path(resource_type, day, year);

    read_to_string(file_path)
}

fn build_path(resource_type: ResourceType, day: &str, year: Year) -> String {
    let format = match resource_type {
        ResourceType::Inputs => "in",
        ResourceType::Outputs => "out",
        #[cfg(test)]
        ResourceType::Examples => "in",
    };

    format!("resources/{}/{}/{}.{}", year, resource_type, day, format)
}
