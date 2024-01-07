use crate::solutions::Solution;

pub struct Day01;

impl Solution for Day01 {
    fn part_one(&self, input: &str) -> String {
        input.lines()
            .map(calculate_line)
            .sum::<u32>()
            .to_string()
    }

    fn part_two(&self, input: &str) -> String {
        input.lines()
            .map(replace_and_calculate)
            .sum::<u32>()
            .to_string()
    }
}

fn replace_and_calculate(words: &str) -> u32 {
    let string = replace_words_to_numbers(words);

    calculate_line(string.as_str())
}

fn replace_words_to_numbers(words: &str) -> String {
    if words.len() < 5 {
        return replace(words);
    }

    words.to_owned()
        .as_bytes()
        .windows(5)
        .map(|part| {
            let string = String::from_utf8_lossy(part).to_string();

            replace(string.as_str())
        }).collect()
}

fn replace(words: &str) -> String {
    words.replace("one", "1")
        .replace("two", "2")
        .replace("three", "3")
        .replace("four", "4")
        .replace("five", "5")
        .replace("six", "6")
        .replace("seven", "7")
        .replace("eight", "8")
        .replace("nine", "9")
}

fn calculate_line(line: &str) -> u32 {
    let first = first_number(line);
    let last = last_number(line);

    first * 10 + last
}

fn first_number(line: &str) -> u32 {
    for c in line.chars() {
        if c.is_numeric() {
            return c.to_digit(10).unwrap();
        }
    }

    unreachable!()
}

fn last_number(line: &str) -> u32 {
    for c in line.chars().rev() {
        if c.is_numeric() {
            return c.to_digit(10).unwrap();
        }
    }

    unreachable!()
}

#[cfg(test)]
mod tests {
    use crate::file_system::read_example;
    use crate::solutions::day01::{calculate_line, Day01, replace_and_calculate};
    use crate::solutions::Solution;

    #[test]
    fn part_one_example_test() {
        let input = read_example("01");

        assert_eq!("142", Day01.part_one(input.as_str()));
    }

    #[test]
    fn part_two_example_test() {
        let input = read_example("01_2");

        assert_eq!("281", Day01.part_two(input.as_str()));
    }

    #[test]
    fn replace_and_calculate_test() {
        assert_eq!(replace_and_calculate("1"), 11);
        assert_eq!(replace_and_calculate("one"), 11);
        assert_eq!(replace_and_calculate("eightwo"), 82);
        assert_eq!(replace_and_calculate("two1nine"), 29);
        assert_eq!(replace_and_calculate("eightwothree"), 83);
        assert_eq!(replace_and_calculate("abcone2threexyz"), 13);
        assert_eq!(replace_and_calculate("xtwone3four"), 24);
        assert_eq!(replace_and_calculate("4nineeightseven2"), 42);
        assert_eq!(replace_and_calculate("zoneight234"), 14);
        assert_eq!(replace_and_calculate("7pqrstsixteen"), 76);
        assert_eq!(replace_and_calculate("fivethreeonezblqnsfk1"), 51);
        assert_eq!(replace_and_calculate("two74119onebtqgnine"), 29);
        assert_eq!(replace_and_calculate("jrjh5vsrxbhsfour3"), 53);
    }

    #[test]
    fn calculate_line_test() {
        assert_eq!(calculate_line("12"), 12);
        assert_eq!(calculate_line("219"), 29);
        assert_eq!(calculate_line("8wo3"), 83);
        assert_eq!(calculate_line("abc123xyz"), 13);
        assert_eq!(calculate_line("x2ne34"), 24);
        assert_eq!(calculate_line("49872"), 42);
        assert_eq!(calculate_line("z1ight234"), 14);
        assert_eq!(calculate_line("7pqrst6teen"), 76);
        assert_eq!(calculate_line("531zblqnsfk1"), 51);
        assert_eq!(calculate_line("2741191btqg9"), 29);
        assert_eq!(calculate_line("jrjh5vsrxbhs43"), 53);
        assert_eq!(calculate_line("jrjhvsrxbhs43"), 43);
        assert_eq!(calculate_line("treb7uchet"), 77);
    }
}
