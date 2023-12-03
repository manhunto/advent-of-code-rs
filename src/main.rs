use std::fs::read_to_string;

fn main() {
    let input = read_to_string("src/01_puzzle.in")
        .expect("Should be able to read this file");

    let lines = input.split("\n");

    let mut sum: i32 = 0;

    for line in lines {
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

    // let output = read_to_string("src/01_example.out")
    //     .expect("Should be able to read this file");
    //
    // assert_eq!(output, sum.to_string());

    println!("{}", sum);
}
