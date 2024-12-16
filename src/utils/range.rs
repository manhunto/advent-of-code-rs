use fmt::Display;
use std::fmt;

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Range {
    start: isize,
    end: isize,
}

impl Range {
    pub fn new(start: isize, end: isize) -> Result<Self, String> {
        if start > end {
            return Err(format!("Start ({}) is less than end ({})", start, end));
        }

        Ok(Self { start, end })
    }

    pub fn with_length(start: isize, len: isize) -> Result<Self, String> {
        Self::new(start, start + len - 1)
    }

    pub fn start(&self) -> isize {
        self.start
    }

    pub fn end(&self) -> isize {
        self.end
    }

    pub fn len(&self) -> isize {
        self.end - self.start + 1
    }

    pub fn move_start_at(&self, start: isize) -> Result<Self, String> {
        Self::with_length(start, self.len())
    }

    pub fn contains(&self, number: isize) -> bool {
        (self.start..=self.end).contains(&number) // todo: optimize me
    }

    pub fn collide(&self, other: &Self) -> bool {
        self.contains(other.start)
            || self.contains(other.end)
            || other.contains(self.start)
            || other.contains(self.end)
    }

    pub fn intersect(&self, other: &Self) -> Result<Self, String> {
        if !self.collide(other) {
            return Err("Cannot intersect for ranges that doesn't collide".to_string());
        }

        Ok(Self::new(self.start.max(other.start), self.end.min(other.end)).unwrap())
    }

    pub fn diff(&self, other: &Self) -> Vec<Self> {
        // other is outside self
        if other.start <= self.start && other.end >= self.end {
            return vec![];
        }

        // other is inside self
        if other.start > self.start && other.end < self.end {
            return vec![
                Self::new(self.start, other.start - 1).unwrap(),
                Self::new(other.end + 1, self.end).unwrap(),
            ];
        }

        let start: isize;
        let end: isize;

        // collide on side
        if self.start < other.start {
            start = self.start.min(other.start);
            end = self.start.max(other.start) - 1;
        } else {
            start = self.end.min(other.end) + 1;
            end = self.end.max(other.end);
        }

        vec![Self::new(start, end).unwrap()]
    }

    pub fn iter(&self) -> impl Iterator<Item = isize> {
        self.start..=self.end
    }

    pub fn rev_iter(&self) -> impl Iterator<Item = isize> {
        self.iter().collect::<Vec<isize>>().into_iter().rev()
    }

    #[allow(dead_code)]
    pub fn is_before(&self, value: isize) -> bool {
        self.start > value
    }

    #[allow(dead_code)]
    pub fn is_after(&self, value: isize) -> bool {
        self.end < value
    }

    pub fn shrink(&self, by: isize) -> Result<Self, String> {
        Self::new(self.start + by, self.end - by)
    }
}

impl Display for Range {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.start, self.end)
    }
}

#[cfg(test)]
mod tests {
    use crate::utils::range::Range;

    #[test]
    fn contains() {
        let range = Range::new(5, 7).unwrap();

        assert!(!range.contains(4));
        assert!(range.contains(5));
        assert!(range.contains(6));
        assert!(range.contains(7));
        assert!(!range.contains(8));
    }

    #[test]
    fn collide() {
        let range = Range::new(3, 5).unwrap();

        assert!(!range.collide(&Range::new(1, 2).unwrap()));
        assert!(range.collide(&Range::new(2, 3).unwrap()));
        assert!(range.collide(&Range::new(3, 5).unwrap()));
        assert!(range.collide(&Range::new(4, 6).unwrap()));
        assert!(range.collide(&Range::new(5, 10).unwrap()));
        assert!(!range.collide(&Range::new(6, 12).unwrap()));
        assert!(range.collide(&Range::new(1, 7).unwrap()));
    }

    #[test]
    fn intersect() {
        let range = Range::new(3, 6).unwrap();

        assert_eq!(
            Range::new(3, 3),
            range.intersect(&Range::new(2, 3).unwrap())
        );
        assert_eq!(
            Range::new(3, 4),
            range.intersect(&Range::new(2, 4).unwrap())
        );
        assert_eq!(
            Range::new(3, 5),
            range.intersect(&Range::new(1, 5).unwrap())
        );
        assert_eq!(
            Range::new(3, 6),
            range.intersect(&Range::new(0, 6).unwrap())
        );
        assert_eq!(
            Range::new(3, 6),
            range.intersect(&Range::new(0, 7).unwrap())
        );
        assert_eq!(
            Range::new(4, 5),
            range.intersect(&Range::new(4, 5).unwrap())
        );
        assert_eq!(
            Range::new(5, 6),
            range.intersect(&Range::new(5, 7).unwrap())
        );
    }
}
