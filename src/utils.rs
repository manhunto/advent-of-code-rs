pub fn replace_words_to_numbers(x: &str) -> String {
    for i in 3..x.len() + 1 {
        let part: &str = &x[0..i];
        let replaced_part = part
            .replace("one", "1")
            .replace("two", "2")
            .replace("three", "3")
            .replace("four", "4")
            .replace("five", "5")
            .replace("six", "6")
            .replace("seven", "7")
            .replace("eight", "8")
            .replace("nine", "9");

        if part != replaced_part.as_str() {
            let rest = &x[i..x.len()];

            return replace_words_to_numbers(format!("{}{}", replaced_part, rest).as_str());
        }
    }

    return String::from(x);
}

pub fn calculate_line(line: &str) -> u32 {
    let mut numbers = Vec::new();

    for char in line.chars() {
        if char.is_numeric() {
            numbers.push(char);
        }
    }

    let first = numbers.first().unwrap().to_digit(10).unwrap();
    let last = numbers.last().unwrap().to_digit(10).unwrap();

    first * 10 + last
}

#[cfg(test)]
mod tests {
    use crate::utils::{calculate_line, replace_words_to_numbers};

    #[test]
    fn replace_words_to_numbers_test() {
        assert_eq!(replace_words_to_numbers("1"), "1");
        assert_eq!(replace_words_to_numbers("one"), "1");
        assert_eq!(replace_words_to_numbers("eightwo"), "8wo");
        assert_eq!(replace_words_to_numbers("two1nine"), "219");
        assert_eq!(replace_words_to_numbers("eightwothree"), "8wo3");
        assert_eq!(replace_words_to_numbers("abcone2threexyz"), "abc123xyz");
        assert_eq!(replace_words_to_numbers("xtwone3four"), "x2ne34");
        assert_eq!(replace_words_to_numbers("4nineeightseven2"), "49872");
        assert_eq!(replace_words_to_numbers("zoneight234"), "z1ight234");
        assert_eq!(replace_words_to_numbers("7pqrstsixteen"), "7pqrst6teen");
        assert_eq!(replace_words_to_numbers("fivethreeonezblqnsfk1"), "531zblqnsfk1");
        assert_eq!(replace_words_to_numbers("two74119onebtqgnine"), "2741191btqg9");
        assert_eq!(replace_words_to_numbers("jrjh5vsrxbhsfour3"), "jrjh5vsrxbhs43");
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
