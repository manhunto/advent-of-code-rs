use std::fs::read_to_string;

pub fn read_input(day: &str) -> String {
    let file = format!("resources/inputs/{}.in", day);

    read(file.clone()).expect(format!("Could not read file! {}", file).as_str())
}

pub fn read_output(day: &str) -> std::io::Result<String> {
    read(format!("resources/outputs/{}.out", day))
}

#[cfg(test)]
pub fn read_example(day: &str) -> String {
    let file = format!("resources/examples/{}.in", day);

    read(file.clone()).expect(format!("Could not read file! {}", file).as_str())
}

fn read(file_path: String) -> std::io::Result<String> {
    read_to_string(file_path)
}
