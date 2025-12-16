pub trait IsInside<T> {
    fn is_inside(&self, value: &T) -> bool;
}

#[allow(dead_code)]
pub trait Intersect<T> {
    fn intersect(&self, value: &T) -> bool;
}
