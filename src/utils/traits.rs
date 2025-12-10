pub trait IsInside<T> {
    fn is_inside(&self, value: &T) -> bool;
}
