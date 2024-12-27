use crate::solutions::Solution;

pub struct Day22;

impl Solution for Day22 {
    fn part_one(&self, input: &str) -> String {
        input
            .lines()
            .map(|line| {
                let initial: usize = line.parse().unwrap();
                let secrets = self.next_secrets(initial, 2000);

                *secrets.last().unwrap()
            })
            .sum::<usize>()
            .to_string()
    }

    fn part_two(&self, _input: &str) -> String {
        String::from("0")
    }
}

impl Day22 {
    fn next_secrets(&self, initial: usize, number_of_secrets: usize) -> Vec<usize> {
        let mut secret = initial;
        let mut next_secrets = Vec::new();

        for _ in 0..number_of_secrets {
            let tmp = secret * 64;
            secret ^= tmp;
            secret %= 16777216;

            let tmp = secret / 32;
            secret ^= tmp;
            secret %= 16777216;

            let tmp = secret * 2048;
            secret ^= tmp;
            secret %= 16777216;

            next_secrets.push(secret);
        }

        next_secrets
    }
}

#[cfg(test)]
mod tests {
    use crate::solutions::year2024::day22::Day22;
    use crate::solutions::Solution;

    const EXAMPLE: &str = r#"1
10
100
2024"#;

    #[test]
    fn part_one_example() {
        assert_eq!("37327623", Day22.part_one(EXAMPLE));
    }

    #[test]
    fn next_secrets() {
        let tmp = Day22.next_secrets(123, 10);
        let mut result = tmp.iter();

        assert_eq!(Some(&15887950), result.next());
        assert_eq!(Some(&16495136), result.next());
        assert_eq!(Some(&527345), result.next());
        assert_eq!(Some(&704524), result.next());
        assert_eq!(Some(&1553684), result.next());
        assert_eq!(Some(&12683156), result.next());
        assert_eq!(Some(&11100544), result.next());
        assert_eq!(Some(&12249484), result.next());
        assert_eq!(Some(&7753432), result.next());
        assert_eq!(Some(&5908254), result.next());
        assert_eq!(None, result.next());
    }
}
