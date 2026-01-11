use crate::solutions::Solution;
use itertools::Itertools;
use std::collections::HashSet;

pub struct Day07;

impl Solution for Day07 {
    fn part_one(&self, input: &str) -> String {
        input
            .lines()
            .filter(|line| Ip::from(*line).supports_tls())
            .count()
            .to_string()
    }

    fn part_two(&self, input: &str) -> String {
        input
            .lines()
            .filter(|line| Ip::from(*line).supports_ssl())
            .count()
            .to_string()
    }
}

struct Ip<'a> {
    parts: Vec<&'a str>,
}

impl<'a> From<&'a str> for Ip<'a> {
    fn from(value: &'a str) -> Self {
        Ip {
            parts: value.split_terminator(['[', ']']).collect(),
        }
    }
}

impl<'a> Ip<'a> {
    fn supports_tls(&self) -> bool {
        let abba_in_supernet = self.supernet_iter().any(Self::abba);
        let abba_in_hypernet = self.hypernet_iter().any(Self::abba);

        abba_in_supernet && !abba_in_hypernet
    }

    fn abba(part: &str) -> bool {
        part.chars().collect_vec().windows(4).any(|window| {
            window[0] == window[3] && window[1] == window[2] && window[0] != window[1]
        })
    }

    fn supports_ssl(&self) -> bool {
        let aba_in_supernet: HashSet<String> = self.supernet_iter().flat_map(Self::aba).collect();
        let aba_in_hypernet: HashSet<String> = self
            .hypernet_iter()
            .flat_map(Self::aba)
            .map(|str| {
                let x: Vec<char> = str.chars().collect();

                format!("{}{}{}", x[1], x[0], x[1])
            })
            .collect();

        aba_in_supernet
            .iter()
            .any(|aba| aba_in_hypernet.contains(aba))
    }

    fn aba(part: &str) -> HashSet<String> {
        part.chars()
            .collect_vec()
            .windows(3)
            .filter(|window| window[0] == window[2] && window[0] != window[1])
            .map(|window| window.iter().collect())
            .collect()
    }

    /// outside any square bracketed sections
    fn supernet_iter(&self) -> impl Iterator<Item = &'a str> + '_ {
        self.parts.iter().step_by(2).copied()
    }

    /// inside any square bracketed sections
    fn hypernet_iter(&self) -> impl Iterator<Item = &'a str> + '_ {
        self.parts.iter().skip(1).step_by(2).copied()
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

    #[test]
    fn ip_supports_ssl() {
        assert!(Ip::from("aba[bab]xyz").supports_ssl());
        assert!(!Ip::from("xyx[xyx]xyx").supports_ssl());
        assert!(Ip::from("aaa[kek]eke").supports_ssl());
        assert!(Ip::from("zazbz[bzb]cdb").supports_ssl());
    }
}
