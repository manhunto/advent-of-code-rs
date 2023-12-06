use std::fs::read_to_string;

pub fn read_input(day: &str) -> String {
    read(format!("resources/inputs/{}.in", day))
}

#[cfg(test)]
pub fn read_example(day: &str) -> String {
    read(format!("resources/examples/{}.in", day))
}

fn read(file_path: String) -> String {
    read_to_string(file_path)
        .expect("Should be able to read this file")
}