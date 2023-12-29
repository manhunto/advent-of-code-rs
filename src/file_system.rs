use std::fs::read_to_string;

pub fn read_input(day: &str) -> String {
    read(format!("resources/inputs/{}.in", day)).unwrap()
}

pub fn read_output(day: &str) -> std::io::Result<String> {
    read(format!("resources/outputs/{}.out", day))
}

#[cfg(test)]
pub fn read_example(day: &str) -> String {
    read(format!("resources/examples/{}.in", day)).unwrap()
}

fn read(file_path: String) -> std::io::Result<String> {
    read_to_string(file_path)
}