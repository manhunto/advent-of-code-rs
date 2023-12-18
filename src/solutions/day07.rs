use std::cmp::Ordering;
use std::collections::HashMap;
use crate::solutions::Solution;

pub struct Day07;

impl Solution for Day07 {
    fn part_one(&self, input: &str) -> String {
        self.solve(&input, |a, b| a.cmp(&b))
    }

    fn part_two(&self, input: &str) -> String {
        self.solve(&input, |a, b| a.cmp_joker_rule(&b))
    }
}

impl Day07 {
    fn solve(&self, input: &str, compare: impl FnMut(&HandWithBid, &HandWithBid) -> Ordering) -> String {
        let mut hands = parse_input(input);

        hands.sort_by(compare);
        hands
            .iter()
            .enumerate()
            .map(|(i, hand)| (i + 1) as i32 * hand.bid)
            .sum::<i32>()
            .to_string()
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

#[derive(Debug, PartialEq, Clone)]
struct Hand {
    cards: Vec<char>,
}

impl Hand {
    fn new(cards: Vec<char>) -> Self {
        Self { cards }
    }

    #[cfg(test)]
    fn from_string(string: &str) -> Self {
        Self::new(string.chars().collect())
    }

    fn recognize(&self) -> Type {
        let chars = self.cards.iter().fold(HashMap::new(), |mut acc, c| {
            *acc.entry(c).or_insert(0) += 1;
            acc
        });

        let mut counted: Vec<i32> = vec![];

        for value in chars.values() {
            counted.push(*value);
        }

        counted.sort();

        let number = counted
            .iter()
            .fold(String::from(""), |acc, elem| format!("{}{}", acc, elem))
            .parse()
            .unwrap(); // todo extract one function (duplicated)

        match number {
            5 => Type::FiveOfKind,
            14 => Type::FourOfKind,
            23 => Type::FullHouse,
            122 => Type::TwoPair,
            113 => Type::ThereOfKind,
            1112 => Type::OnePair,
            11111 => Type::HighCard,
            _ => panic!("{}", format!("Unrecognized type for hand: {:?} ({:?})", self.cards, counted))
        }
    }

    fn cmp(&self, other: &Self) -> Ordering {
        let other_type = other.recognize() as i32;
        let my_type = self.recognize() as i32;

        return if my_type > other_type {
            Ordering::Greater
        } else if my_type < other_type {
            Ordering::Less
        } else {
            self.cmp_the_same(other, 11)
        };
    }

    fn cmp_joker_rule(&self, other: &Self) -> Ordering {
        let other_type = other.recognize_joker_rule() as i32;
        let my_type = self.recognize_joker_rule() as i32;

        return if my_type > other_type {
            Ordering::Greater
        } else if my_type < other_type {
            Ordering::Less
        } else {
            self.cmp_the_same(other, 0)
        };
    }

    fn recognize_joker_rule(&self) -> Type {
        if !self.cards.contains(&'J') {
            return self.recognize();
        }

        let mut new_cards: Vec<Self> = vec![];
        for card in &self.cards {
            let tmp = self.cards.clone();
            let new_card: Vec<char> = tmp
                .iter()
                .map(|t| {
                    match t {
                        &'J' => card.clone(),
                        _ => t.clone()
                    }
                })
                .collect();
            new_cards.push(Hand::new(new_card))
        }

        new_cards.sort_by(|a, b| a.cmp(&b));

        let x = new_cards.last().expect(&*format!("{:?}", self.cards));

        x.recognize()
    }

    fn cmp_the_same(&self, other: &Self, joker_weight: i32) -> Ordering {
        for (i, my_c) in self.cards.iter().enumerate() {
            let other_c = other.cards.get(i).unwrap();

            let my_c_int = label_to_int(my_c, joker_weight);
            let other_c_int = label_to_int(other_c, joker_weight);

            let ordering = my_c_int.cmp(&other_c_int);
            if ordering.is_eq() {
                continue;
            }

            return ordering;
        }

        Ordering::Equal
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

    fn cmp(&self, other: &Self) -> Ordering {
        self.hand.cmp(&other.hand)
    }

    fn cmp_joker_rule(&self, other: &Self) -> Ordering {
        self.hand.cmp_joker_rule(&other.hand)
    }
}

#[derive(PartialEq, Debug)]
enum Type {
    FiveOfKind = 6,
    FourOfKind = 5,
    FullHouse = 4,
    ThereOfKind = 3,
    TwoPair = 2,
    OnePair = 1,
    HighCard = 0,
}

fn label_to_int(label: &char, joker_weight: i32) -> i32 {
    match label {
        'A' => 14,
        'K' => 13,
        'Q' => 12,
        'J' => joker_weight,
        'T' => 10,
        '9' => 9,
        '8' => 8,
        '7' => 7,
        '6' => 6,
        '5' => 5,
        '4' => 4,
        '3' => 3,
        '2' => 2,
        '1' => 1,
        _ => panic!("{}", format!("Unrecognized label: {}", label))
    }
}

#[cfg(test)]
mod tests {
    use std::cmp::Ordering;
    use crate::file_system::read_example;
    use crate::solutions::day07::{Day07, Hand, Type};
    use crate::solutions::Solution;

    #[test]
    fn part_one_example_test() {
        let input = read_example("07");

        assert_eq!("6440", Day07.part_one(&input.as_str()));
    }

    #[test]
    fn part_two_example_test() {
        let input = read_example("07");

        assert_eq!("5905", Day07.part_two(&input.as_str()));
    }

    #[test]
    fn hand_recognize_test() {
        assert_eq!(Type::FiveOfKind, Hand::from_string("AAAAA").recognize());
        assert_eq!(Type::FiveOfKind, Hand::from_string("KKKKK").recognize());
        assert_eq!(Type::FourOfKind, Hand::from_string("KK8KK").recognize());
        assert_eq!(Type::FourOfKind, Hand::from_string("33332").recognize());
        assert_eq!(Type::FullHouse, Hand::from_string("Q3Q33").recognize());
        assert_eq!(Type::FullHouse, Hand::from_string("98989").recognize());
        assert_eq!(Type::ThereOfKind, Hand::from_string("98488").recognize());
        assert_eq!(Type::ThereOfKind, Hand::from_string("11123").recognize());
        assert_eq!(Type::TwoPair, Hand::from_string("22339").recognize());
        assert_eq!(Type::TwoPair, Hand::from_string("QKQK4").recognize());
        assert_eq!(Type::OnePair, Hand::from_string("A23A4").recognize());
        assert_eq!(Type::OnePair, Hand::from_string("Q78QT").recognize());
        assert_eq!(Type::HighCard, Hand::from_string("12345").recognize());
        assert_eq!(Type::HighCard, Hand::from_string("531QK").recognize());
    }

    #[test]
    fn hand_cmp_test() {
        assert_eq!(Ordering::Greater, Hand::from_string("AAAAA").cmp(&Hand::from_string("KK8KK")));
        assert_eq!(Ordering::Less, Hand::from_string("QQQQ4").cmp(&Hand::from_string("KKKKK")));
        assert_eq!(Ordering::Equal, Hand::from_string("QQQQQ").cmp(&Hand::from_string("QQQQQ")));
        assert_eq!(Ordering::Less, Hand::from_string("QQQQQ").cmp(&Hand::from_string("KKKKK")));
        assert_eq!(Ordering::Greater, Hand::from_string("KKKKK").cmp(&Hand::from_string("TTTTT")));
        assert_eq!(Ordering::Less, Hand::from_string("8KQAJ").cmp(&Hand::from_string("91234")));
    }

    #[test]
    fn hand_recognize_joker_rule() {
        assert_eq!(Type::FourOfKind, Hand::from_string("QJJQ2").recognize_joker_rule());
        assert_eq!(Type::FourOfKind, Hand::from_string("T55J5").recognize_joker_rule());
        assert_eq!(Type::FourOfKind, Hand::from_string("KTJJT").recognize_joker_rule());
        assert_eq!(Type::FourOfKind, Hand::from_string("QQQJA").recognize_joker_rule());
    }
}
