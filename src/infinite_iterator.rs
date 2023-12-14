#[derive(PartialEq, Debug)]
pub struct InfiniteIterator<T> {
    items: Vec<T>,
    index: i32,
}

impl<T> InfiniteIterator<T> {
    pub fn new(items: Vec<T>) -> Self {
        Self {
            items,
            index: 0,
        }
    }
    pub fn next(&mut self) -> &T {
        let len: i32 = self.items.len() as i32;
        let index: i32 = self.index % len;
        let item: &T = self.items.get(index as usize).unwrap();

        self.index += 1;

        item
    }
}

#[cfg(test)]
mod tests {
    use crate::infinite_iterator::InfiniteIterator;

    #[test]
    fn next() {
        let vec: Vec<char> = vec!['R', 'L'];
        let mut iter: InfiniteIterator<char> = InfiniteIterator::new(vec);

        assert_eq!(&'R', iter.next());
        assert_eq!(&'L', iter.next());
        assert_eq!(&'R', iter.next());
        assert_eq!(&'L', iter.next());
        assert_eq!(&'R', iter.next());
    }
}