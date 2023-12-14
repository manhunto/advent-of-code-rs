use crate::solutions::Solution;

pub struct Day07;

impl Solution for Day07 {
    fn part_one(&self, input: &str) -> String {
        let hands = parse_input(input);

        println!("{:?}", hands);

       String::from("0")
    }

    fn part_two(&self, input: &str) -> String {
        String::from("0")
    }
}

fn parse_input(input: &str) -> Vec<HandWithBid> {
    input
        .lines()
        .map(|line| {
            let mut parts = line.split_whitespace();

            let cards: Vec<char> = parts.next().unwrap().chars().collect();
            let bid: i32 = parts.next().unwrap().parse().unwrap();

            HandWithBid::new(Hand::new(cards), bid)
        })
        .collect()
}

#[derive(Debug)]
struct Hand {
    cards: Vec<char>
}

impl Hand {
    fn new(cards: Vec<char>) -> Self {
        Self {
            cards
        }
    }
}

#[derive(Debug)]
struct HandWithBid {
    hand: Hand,
    bid: i32,
}

impl HandWithBid {
    fn new(hand: Hand, bid: i32) -> Self {
        Self {
            hand,
            bid,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::file_system::read_example;
    use crate::solutions::day07::{Day07};
    use crate::solutions::Solution;

    #[test]
    fn part_one_example_test() {
        let input = read_example("07");

        assert_eq!("6440", Day07.part_one(&input.as_str()));
    }
}
