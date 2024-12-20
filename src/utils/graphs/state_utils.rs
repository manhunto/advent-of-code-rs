use std::cmp::Ordering;

#[derive(PartialEq, Eq)]
pub struct State<T> {
    pub node: T,
    pub cost: usize,
}

impl<T> State<T> {
    pub fn new(node: T, cost: usize) -> Self {
        Self { node, cost }
    }
}

impl<T> PartialOrd for State<T>
where
    T: PartialEq + Ord,
{
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<T> Ord for State<T>
where
    T: Eq + Ord,
{
    fn cmp(&self, other: &Self) -> Ordering {
        other
            .cost
            .cmp(&self.cost)
            .then_with(|| self.node.cmp(&other.node))
    }
}
