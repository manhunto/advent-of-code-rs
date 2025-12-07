use fmt::Display;
use std::fmt;
use std::ops::RangeInclusive;

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
        (self.start..=self.end).contains(&number)
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

    /// Calculates the difference between two ranges.
    ///
    /// This method returns a vector of ranges representing the parts of `self`
    /// that are not present in `other`.
    ///
    /// # Cases:
    /// - If `other` completely contains `self`, the result is an empty vector.
    /// - If `self` completely contains `other`, the result is a vector with two ranges.
    /// - If they partially overlap, the result is a vector with one range.
    /// - If they do not overlap, the result is a vector containing `self`.
    ///
    /// # Example
    ///
    /// ```
    /// use crate::utils::range::Range;
    ///
    /// let range = Range::new(3, 8).unwrap();
    ///
    /// // Case where `other` is inside `self`
    /// let other1 = Range::new(5, 6).unwrap();
    /// assert_eq!(
    ///     vec![Range::new(3, 4).unwrap(), Range::new(7, 8).unwrap()],
    ///     range.diff(&other1)
    /// );
    ///
    /// // Case where `other` overlaps on the right
    /// let other2 = Range::new(7, 10).unwrap();
    /// assert_eq!(
    ///     vec![Range::new(3, 6).unwrap()],
    ///     range.diff(&other2)
    /// );
    ///
    /// // Case where `other` does not overlap
    /// let other3 = Range::new(10, 12).unwrap();
    /// assert_eq!(
    ///     vec![range],
    ///     range.diff(&other3)
    /// );
    ///
    /// // Case where `other` completely contains `self`
    /// let other4 = Range::new(1, 10).unwrap();
    /// assert_eq!(
    ///     Vec::<Range>::new(),
    ///     range.diff(&other4)
    /// );
    /// ```
    pub fn diff(&self, other: &Self) -> Vec<Self> {
        let mut result = Vec::new();

        // Part of self before other starts
        if self.start < other.start {
            let end = self.end.min(other.start - 1);
            if self.start <= end {
                result.push(Self::new(self.start, end).unwrap());
            }
        }

        // Part of self after other ends
        if self.end > other.end {
            let start = self.start.max(other.end + 1);
            if start <= self.end {
                result.push(Self::new(start, self.end).unwrap());
            }
        }

        result
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

    pub fn _shrink(&self, by: isize) -> Result<Self, String> {
        Self::new(self.start + by, self.end - by)
    }
}

impl Display for Range {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.start, self.end)
    }
}

impl From<RangeInclusive<isize>> for Range {
    fn from(value: RangeInclusive<isize>) -> Self {
        Self::new(*value.start(), *value.end()).unwrap()
    }
}

impl TryFrom<(&str, &str)> for Range {
    type Error = String;

    fn try_from(value: (&str, &str)) -> Result<Self, Self::Error> {
        let start = value
            .0
            .parse::<isize>()
            .map_err(|e| format!("Error parsing start value: {}", e))?;

        let end = value
            .1
            .parse::<isize>()
            .map_err(|e| format!("Error parsing end value: {}", e))?;

        Self::new(start, end)
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

    #[test]
    fn diff() {
        let range = Range::new(3, 6).unwrap();

        assert_eq!(
            vec![Range::new(3, 3).unwrap()],
            range.diff(&Range::new(4, 6).unwrap())
        );

        assert_eq!(
            vec![Range::new(3, 3).unwrap(), Range::new(6, 6).unwrap()],
            range.diff(&Range::new(4, 5).unwrap())
        );

        // Case where other does not overlap and is after self
        assert_eq!(vec![range], range.diff(&Range::new(10, 14).unwrap()));

        // Case where other does not overlap and is before self
        assert_eq!(vec![range], range.diff(&Range::new(0, 1).unwrap()));

        // Case where other completely contains self
        assert_eq!(Vec::<Range>::new(), range.diff(&Range::new(2, 7).unwrap()));

        // Case where self and other are identical
        assert_eq!(Vec::<Range>::new(), range.diff(&Range::new(3, 6).unwrap()));

        // Case where other overlaps on the left
        assert_eq!(
            vec![Range::new(5, 6).unwrap()],
            range.diff(&Range::new(1, 4).unwrap())
        );
    }
}
