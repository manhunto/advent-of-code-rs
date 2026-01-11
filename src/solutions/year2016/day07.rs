use crate::solutions::Solution;
use itertools::Itertools;

pub struct Day07;

impl Solution for Day07 {
    fn part_one(&self, input: &str) -> String {
        input
            .lines()
            .filter(|line| Ip::from(*line).supports_tls())
            .count()
            .to_string()
    }

    fn part_two(&self, _input: &str) -> String {
        String::from("0")
    }
}

struct Ip<'a> {
    ip: &'a str,
}

impl<'a> From<&'a str> for Ip<'a> {
    fn from(ip: &'a str) -> Self {
        Ip { ip }
    }
}

impl<'a> Ip<'a> {
    fn supports_tls(&self) -> bool {
        let parts: Vec<&str> = self.ip.split_terminator(['[', ']']).collect();

        let non_bracket = parts.iter().step_by(2).any(|part| Self::abba(part));
        let bracket = parts.iter().skip(1).step_by(2).any(|part| Self::abba(part));

        non_bracket && !bracket
    }

    fn abba(part: &str) -> bool {
        part.chars().collect_vec().windows(4).any(|window| {
            window[0] == window[3] && window[1] == window[2] && window[0] != window[1]
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = r#"abba[mnop]qrst
abcd[bddb]xyyx
aaaa[qwer]tyui
ioxxoj[asdfgh]zxcvbn"#;

    #[test]
    fn part_one_example() {
        assert_eq!("2", Day07.part_one(EXAMPLE));
    }

    #[test]
    fn ip_supports_tls() {
        assert!(Ip::from("abba[mnop]qrst").supports_tls());
        assert!(!Ip::from("abcd[bddb]xyyx").supports_tls());
        assert!(!Ip::from("aaaa[qwer]tyui").supports_tls());
        assert!(Ip::from("ioxxoj[asdfgh]zxcvbn").supports_tls());
    }

    #[test]
    fn ip_abba() {
        assert!(Ip::abba("abba"));
        assert!(Ip::abba("ioxxoj"));
        assert!(!Ip::abba("aaaa"));
    }
}
