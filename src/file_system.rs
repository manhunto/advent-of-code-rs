use std::fmt::{Display, Formatter};
use std::fs::read_to_string;
use crate::year::Year;

enum ResourceType {
    Inputs,
    Outputs,
    #[cfg(test)]
    Examples
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

pub fn read_input(day: &str, year: Year) -> String {
    read(ResourceType::Inputs, day, year).unwrap()
}

pub fn read_output(day: &str, year: Year) -> std::io::Result<String> {
    read(ResourceType::Outputs, day, year)
}

#[cfg(test)]
pub fn read_example(day: &str, year: Year) -> String {
    read(ResourceType::Examples, day, year).unwrap()
}

fn read(resource_type: ResourceType, day: &str, year: Year) -> std::io::Result<String> {
    let file_path = format!("resources/{}/{}/{}.in", year, resource_type, day);

    read_to_string(file_path)
}
