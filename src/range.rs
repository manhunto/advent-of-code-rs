pub struct Range {
    start: u64,
    end: u64,
}

impl Range {
    pub fn new(start: u64, end: u64) -> Result<Range, &'static str> {
        if start > end {
            return Err("Start is less than end");
        }

        return Ok(Self {
            start,
            end,
        });
    }

    pub fn with_length(start: u64, len: u64) -> Result<Range, &'static str> {
        Self::new(start, start + len)
    }

    pub fn iter(&self) -> Vec<u64> {
        let range = self.start..self.end;
        range.collect()
    }

    pub fn is_in_range(&self, number: u64) -> bool {
        self.start <= number && self.end >= number
    }

    pub fn collide(&self, other: Self) -> bool {
        self.is_in_range(other.start)
            || self.is_in_range(other.end)
            || other.is_in_range(self.start)
            || other.is_in_range(self.end)
    }

    // pub fn intersect(&self, other: Self) -> Self {
    //
    // }
}

#[cfg(test)]
mod tests {
    use crate::range::Range;

    #[test]
    fn is_in_range() {
        let range = Range::new(5, 7).unwrap();

        assert!(!range.is_in_range(4));
        assert!(range.is_in_range(5));
        assert!(range.is_in_range(6));
        assert!(range.is_in_range(7));
        assert!(!range.is_in_range(8));
    }

    #[test]
    fn collide() {
        let range = Range::new(3, 5).unwrap();

        assert!(!range.collide(Range::new(1, 2).unwrap()));
        assert!(range.collide(Range::new(2, 3).unwrap()));
        assert!(range.collide(Range::new(3, 5).unwrap()));
        assert!(range.collide(Range::new(4, 6).unwrap()));
        assert!(range.collide(Range::new(5, 10).unwrap()));
        assert!(!range.collide(Range::new(6, 12).unwrap()));
        assert!(range.collide(Range::new(1, 7).unwrap()));
    }
}