use std::fs::read_to_string;

fn main() {
    let input = read_to_string("src/01_puzzle.in")
        .expect("Should be able to read this file");

    let lines: Vec<&str> = input.split("\n").collect();

    let part_one: i32 = part_one(&lines);
    let part_two: i32 = part_two(&lines);

    println!("Part one: {}\nPart two: {}", part_one, part_two);
}

fn part_one(_lines: &Vec<&str>) -> i32 {
    let mut sum: i32 = 0;

    for line in _lines {
        let mut numbers = Vec::new();

        for char in line.chars() {
            if char.is_numeric() {
                numbers.push(char);
            }
        }

        let number = format!("{}{}", numbers.first().unwrap(), numbers.last().unwrap());
        let number_as_int: i32 = number.parse().unwrap();

        sum += number_as_int;
    }

    sum
}

fn part_two(_lines: &Vec<&str>) -> i32 {
    0
}
