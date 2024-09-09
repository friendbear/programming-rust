trait StringSet {
    fn new() -> Self;
    fn from_slice(slice: &[&str]) -> Self;
    fn contains(&self, value: &str) -> bool;
    fn add(&mut self, value: &str) -> bool;
}