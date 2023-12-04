pub fn replace_words_to_numbers(x: &str) -> String {
    for i in 1..x.len() + 1 {
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
            .replace("nine", "9")
            ;

        if part != replaced_part.as_str() {
            let rest = &x[i..x.len()];

            return replace_words_to_numbers(format!("{}{}", replaced_part, rest).as_str());
        }
    }

    return String::from(x);
}

pub fn calculate_line(line: &str) -> i32 {
    let mut numbers = Vec::new();

    for char in line.chars() {
        if char.is_numeric() {
            numbers.push(char);
        }
    }

    let number = format!("{}{}", numbers.first().unwrap(), numbers.last().unwrap());

    number.parse().unwrap()
}

#[cfg(test)]
mod tests {
    use crate::utils::replace_words_to_numbers;

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
    }
}
