use crate::solutions::Solution;
use itertools::FoldWhile::{Continue, Done};
use itertools::Itertools;
use md5::{compute, Digest};

pub struct Day05;

impl Solution for Day05 {
    fn part_one(&self, input: &str) -> String {
        self.iter(input.trim())
            .take(8)
            .map(|str| str.chars().nth(5).unwrap())
            .collect::<String>()
    }

    fn part_two(&self, input: &str) -> String {
        self.iter(input.trim())
            .fold_while(['_'; 8], |mut acc, str| {
                let position = &str[5..6];

                if let Ok(pos) = position.parse::<usize>() {
                    if pos < acc.len() && acc[pos] == '_' {
                        acc[pos] = str.chars().nth(6).unwrap();
                    }
                }

                if acc.contains(&'_') {
                    return Continue(acc);
                }

                Done(acc)
            })
            .into_inner()
            .iter()
            .collect()
    }
}

impl Day05 {
    fn iter<'a>(&'a self, door_id: &'a str) -> impl Iterator<Item = String> + 'a {
        let door_id_bytes = door_id.as_bytes();

        (0u64..)
            .map(move |n| {
                let mut input = door_id_bytes.to_vec();
                input.extend_from_slice(n.to_string().as_bytes());
                compute(&input)
            })
            .filter(|digest| self.starts_with_five_zeros(digest))
            .map(|digest| format!("{:x}", digest))
    }

    /// Checks if an MD5 digest starts with five leading zeros in hexadecimal.
    ///
    /// # How it works
    /// An MD5 digest is 16 bytes (128 bits). When displayed in hex, each byte
    /// becomes 2 hex characters, so `digest[0]` represents the first 2 hex chars.
    ///
    /// The condition breaks down as:
    /// - `digest[0] == 0`: First byte is 0x00 → "00" in hex
    /// - `digest[1] == 0`: Second byte is 0x00 → "00" in hex
    /// - `digest[2] < 16`: Third byte is 0x00-0x0F → "00"-"0f" in hex
    ///
    /// A byte can hold values 0-255 (0x00-0xFF):
    /// - 0-15 (0x00-0x0F) → displays as "00" through "0f" (first hex digit is 0)
    /// - 16-255 (0x10-0xFF) → displays as "10" through "ff" (first hex digit is 1-f)
    ///
    /// So `digest[2] < 16` ensures the third byte's first hex character is "0",
    /// giving the fifth leading zero.
    ///
    /// # Examples
    /// ```
    /// // Hash bytes: [0, 0, 7, ...] → Hex: "00007..." ✓ (five zeros)
    /// // Hash bytes: [0, 0, 17, ...] → Hex: "000011..." ✗ (only four zeros)
    /// ```
    ///
    /// This approach is much faster than converting to a string and checking
    /// `starts_with("00000")`.
    fn starts_with_five_zeros(&self, digest: &Digest) -> bool {
        digest[0] == 0 && digest[1] == 0 && digest[2] < 16
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use md5::Digest;

    // todo: it is very slow
    // #[test]
    // fn part_one_example() {
    //     assert_eq!("18f47a30", Day05.part_one("abc"));
    // }
    //
    // #[test]
    // fn part_two_example() {
    //     assert_eq!("05ace8e3", Day05.part_two("abc"));
    // }

    #[test]
    fn test_starts_with_five_zeros_true() {
        // digest[2] = 0 → "000000..." (six zeros)
        let digest = Digest([0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]);
        assert!(Day05.starts_with_five_zeros(&digest));

        // digest[2] = 7 → "00007..." (five zeros)
        let digest = Digest([0, 0, 7, 234, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]);
        assert!(Day05.starts_with_five_zeros(&digest));

        // digest[2] = 15 → "0000f..." (five zeros)
        let digest = Digest([0, 0, 15, 255, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]);
        assert!(Day05.starts_with_five_zeros(&digest));
    }

    #[test]
    fn test_starts_with_five_zeros_false() {
        // digest[2] = 16 → "000010..." (only four zeros)
        let digest = Digest([0, 0, 16, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]);
        assert!(!Day05.starts_with_five_zeros(&digest));

        // digest[1] != 0 → "0001..." (only three zeros)
        let digest = Digest([0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]);
        assert!(!Day05.starts_with_five_zeros(&digest));

        // digest[0] != 0 → "01..." (only one zero)
        let digest = Digest([1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]);
        assert!(!Day05.starts_with_five_zeros(&digest));

        // digest[2] = 255 → "0000ff..." (only four zeros)
        let digest = Digest([0, 0, 255, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]);
        assert!(!Day05.starts_with_five_zeros(&digest));
    }

    #[test]
    fn test_verify_with_actual_formatting() {
        let digest_five_zeros = Digest([0, 0, 7, 234, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]);
        let hex = format!("{:x}", digest_five_zeros);
        assert!(hex.starts_with("00000"));

        let digest_four_zeros = Digest([0, 0, 16, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]);
        let hex = format!("{:x}", digest_four_zeros);
        assert!(!hex.starts_with("00000"));
        assert!(hex.starts_with("0000"));
    }
}
