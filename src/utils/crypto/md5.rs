use md5::Digest;

pub trait DigestExt {
    fn starts_with_five_zeros(&self) -> bool;
    fn starts_with_six_zeros(&self) -> bool;
}

impl DigestExt for Digest {
    fn starts_with_five_zeros(&self) -> bool {
        self.0[0] == 0 && self.0[1] == 0 && self.0[2] < 16
    }

    fn starts_with_six_zeros(&self) -> bool {
        self.0[0] == 0 && self.0[1] == 0 && self.0[2] == 0
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use md5::Digest;

    #[test]
    fn test_five_zeros_true() {
        // digest[2] = 0 → "000000..." (actually six zeros, but passes 5)
        let digest = Digest([0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]);
        assert!(digest.starts_with_five_zeros());

        // digest[2] = 7 → "00007..." (exactly five zeros)
        let digest = Digest([0, 0, 7, 234, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]);
        assert!(digest.starts_with_five_zeros());

        // digest[2] = 15 → "0000f..." (exactly five zeros)
        let digest = Digest([0, 0, 15, 255, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]);
        assert!(digest.starts_with_five_zeros());
    }

    #[test]
    fn test_five_zeros_false() {
        // digest[2] = 16 → "000010..." (only four zeros)
        let digest = Digest([0, 0, 16, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]);
        assert!(!digest.starts_with_five_zeros());

        // digest[1] != 0 → "0001..." (only three zeros)
        let digest = Digest([0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]);
        assert!(!digest.starts_with_five_zeros());
    }

    #[test]
    fn test_six_zeros_true() {
        // digest[2] = 0 → "000000..." (exactly six zeros)
        let digest = Digest([0, 0, 0, 234, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]);
        assert!(digest.starts_with_six_zeros());

        // All zeros
        let digest = Digest([0; 16]);
        assert!(digest.starts_with_six_zeros());
    }

    #[test]
    fn test_six_zeros_false() {
        // digest[2] = 15 → "0000f..." (only five zeros)
        let digest = Digest([0, 0, 15, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]);
        assert!(!digest.starts_with_six_zeros());

        // digest[2] = 1 → "000001..." (only five zeros)
        let digest = Digest([0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]);
        assert!(!digest.starts_with_six_zeros());
    }

    #[test]
    fn test_verify_with_actual_formatting() {
        // Verify 5 zeros logic matches hex formatting
        let digest = Digest([0, 0, 7, 234, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]);
        let hex = format!("{:x}", digest);
        assert_eq!(hex.starts_with("00000"), digest.starts_with_five_zeros());

        // Verify 6 zeros logic matches hex formatting
        let digest = Digest([0, 0, 0, 234, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]);
        let hex = format!("{:x}", digest);
        assert_eq!(hex.starts_with("000000"), digest.starts_with_six_zeros());
    }
}
