use crate::solutions::Solution;

pub struct Day10;

impl Solution for Day10 {
    fn part_one(&self, input: &str) -> String {
        look_and_say_n_times(input.trim(), 40).len().to_string()
    }

    fn part_two(&self, input: &str) -> String {
        look_and_say_n_times(input.trim(), 50).len().to_string()
    }
}

fn look_and_say_n_times(input: &str, iterations: usize) -> String {
    let mut numbers: Vec<u8> = input
        .bytes()
        .map(|b| b - b'0') // 53 (ASCII for 5) - 48 (ASCII for 0) = 5 ✓
        .collect();

    for _ in 0..iterations {
        numbers = look_and_say(&numbers);
    }

    numbers.iter().map(|&n| (n + b'0') as char).collect()
}

fn look_and_say(numbers: &[u8]) -> Vec<u8> {
    if numbers.is_empty() {
        return Vec::new();
    }

    let mut result = Vec::with_capacity(numbers.len() * 3 / 2);
    let chunks = numbers.chunk_by(|a, b| a == b);

    for chunk in chunks {
        result.push(chunk.len() as u8);
        result.push(chunk[0]);
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_look_and_say() {
        assert_eq!("11", look_and_say_n_times("1", 1));
        assert_eq!("21", look_and_say_n_times("11", 1));
        assert_eq!("1211", look_and_say_n_times("21", 1));
        assert_eq!("111221", look_and_say_n_times("1211", 1));
        assert_eq!("312211", look_and_say_n_times("111221", 1));
    }
}
