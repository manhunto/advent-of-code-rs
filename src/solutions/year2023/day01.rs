use crate::solutions::Solution;

pub struct Day01;

impl Solution for Day01 {
    fn part_one(&self, input: &str) -> String {
        input.lines().map(calculate_line).sum::<u32>().to_string()
    }

    fn part_two(&self, input: &str) -> String {
        input
            .lines()
            .map(calculate_line_with_numbers_in_words)
            .sum::<u32>()
            .to_string()
    }
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

fn calculate_line_with_numbers_in_words(words: &str) -> u32 {
    let first = first_number_with_words(words);
    let last = last_number_with_words(words);

    first * 10 + last
}

fn first_number_with_words(words: &str) -> u32 {
    for i in 0..words.len() {
        let c = words.chars().nth(i).unwrap();
        if c.is_numeric() {
            return c.to_digit(10).unwrap();
        }

        let from = i;
        let to = (i + 5).min(words.len());

        if let Some(digit) = recognize_number_in_words(&words[from..to]) {
            return digit;
        }
    }

    unreachable!()
}

fn last_number_with_words(words: &str) -> u32 {
    for i in (0..words.len()).rev() {
        let c = words.chars().nth(i).unwrap();
        if c.is_numeric() {
            return c.to_digit(10).unwrap();
        }

        let from = 0.max(i - 1);
        let to = (i + 4).min(words.len());

        if let Some(digit) = recognize_number_in_words(&words[from..to]) {
            return digit;
        }
    }

    unreachable!()
}

fn recognize_number_in_words(words: &str) -> Option<u32> {
    if words.len() >= 3 {
        match &words[..3] {
            "one" => return Some(1),
            "two" => return Some(2),
            "six" => return Some(6),
            _ => {}
        }
    }

    if words.len() >= 4 {
        match &words[..4] {
            "four" => return Some(4),
            "five" => return Some(5),
            "nine" => return Some(9),
            _ => {}
        }
    }

    if words.len() >= 5 {
        match &words[..5] {
            "three" => return Some(3),
            "seven" => return Some(7),
            "eight" => return Some(8),
            _ => {}
        }
    }

    None
}

#[cfg(test)]
mod tests {
    use crate::solutions::year2023::day01::{
        calculate_line, calculate_line_with_numbers_in_words, Day01,
    };
    use crate::solutions::year2023::read_2023_example;
    use crate::solutions::Solution;

    #[test]
    fn part_one_example_test() {
        let input = read_2023_example("01");

        assert_eq!("142", Day01.part_one(input.as_str()));
    }

    #[test]
    fn part_two_example_test() {
        let input = read_2023_example("01_2");

        assert_eq!("281", Day01.part_two(input.as_str()));
    }

    #[test]
    fn calculate_line_with_numbers_in_words_test() {
        assert_eq!(calculate_line_with_numbers_in_words("1"), 11);
        assert_eq!(calculate_line_with_numbers_in_words("one"), 11);
        assert_eq!(calculate_line_with_numbers_in_words("eightwo"), 82);
        assert_eq!(calculate_line_with_numbers_in_words("two1nine"), 29);
        assert_eq!(calculate_line_with_numbers_in_words("eightwothree"), 83);
        assert_eq!(calculate_line_with_numbers_in_words("abcone2threexyz"), 13);
        assert_eq!(calculate_line_with_numbers_in_words("xtwone3four"), 24);
        assert_eq!(calculate_line_with_numbers_in_words("4nineeightseven2"), 42);
        assert_eq!(calculate_line_with_numbers_in_words("zoneight234"), 14);
        assert_eq!(calculate_line_with_numbers_in_words("7pqrstsixteen"), 76);
        assert_eq!(
            calculate_line_with_numbers_in_words("fivethreeonezblqnsfk1"),
            51
        );
        assert_eq!(
            calculate_line_with_numbers_in_words("two74119onebtqgnine"),
            29
        );
        assert_eq!(
            calculate_line_with_numbers_in_words("jrjh5vsrxbhsfour3"),
            53
        );
        assert_eq!(
            calculate_line_with_numbers_in_words("vrpplrtqxvssgnvdf8"),
            88
        );
        assert_eq!(calculate_line_with_numbers_in_words("z5"), 55);
        assert_eq!(calculate_line_with_numbers_in_words("82dlnzszhpvjftdt"), 82);
        assert_eq!(calculate_line_with_numbers_in_words("3five5"), 35);
        assert_eq!(calculate_line_with_numbers_in_words("two3hj"), 23);
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
