use crate::solutions::Solution;

pub struct Day10;

impl Solution for Day10 {
    fn part_one(&self, input: &str) -> String {
        self.look_and_say_for_string(input, 40).len().to_string()
    }

    fn part_two(&self, input: &str) -> String {
        self.look_and_say_for_string(input, 50).len().to_string()
    }
}

impl Day10 {
    fn look_and_say_for_string(&self, input: &str, times: usize) -> String {
        let mut numbers: Vec<u8> = input
            .trim()
            .chars()
            .map(|c| c.to_string().parse::<u8>().unwrap())
            .collect();

        for _ in 0..times {
            numbers = self.look_and_say(&numbers);
        }

        numbers.iter().map(|u| u.to_string()).collect::<String>()
    }

    fn look_and_say(&self, numbers: &[u8]) -> Vec<u8> {
        let mut current = numbers[0];
        let mut count = 1u8;
        let mut index = 1;
        let mut vec: Vec<u8> = Vec::new();

        while let Some(value) = numbers.get(index) {
            if *value == current {
                count += 1;
                index += 1;
                continue;
            } else {
                vec.push(count);
                vec.push(current);

                count = 1;
                index += 1;
                current = *value;
            }
        }

        vec.push(count);
        vec.push(current);

        vec
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn look_and_say() {
        assert_eq!("11", Day10.look_and_say_for_string("1", 1));
        assert_eq!("21", Day10.look_and_say_for_string("11", 1));
        assert_eq!("1211", Day10.look_and_say_for_string("21", 1));
        assert_eq!("111221", Day10.look_and_say_for_string("1211", 1));
        assert_eq!("312211", Day10.look_and_say_for_string("111221", 1));
    }
}
