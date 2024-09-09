use std::collections::HashSet;

trait StringSet {
    fn new() -> Self
    where
        Self: Sized;
    fn from_slice(slice: &[&str]) -> Self
    where
        Self: Sized;
    fn contains(&self, value: &str) -> bool;
    fn add(&mut self, value: &str) -> bool;
}

impl StringSet for HashSet<String> {
    fn new() -> Self {
        HashSet::new()
    }

    fn from_slice(slice: &[&str]) -> Self {
        let mut set = HashSet::new();
        for &item in slice {
            set.insert(item.to_string());
        }
        set
    }

    fn contains(&self, value: &str) -> bool {
        self.contains(value)
    }

    fn add(&mut self, value: &str) -> bool {
        self.insert(value.to_string())
    }
}