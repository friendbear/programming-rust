use std::ops::Deref;
use std::ops::DerefMut;
pub mod drop;
pub mod sized;

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

pub struct Selector<T> {
    pub elements: Vec<T>,
    pub current: usize,
}

impl<T> Deref for Selector<T> {
    type Target = T;
    fn deref(&self) -> &T {
        &self.elements[self.current]
    }
}
impl<T> DerefMut for Selector<T> {
    fn deref_mut(&mut self) -> &mut T {
        &mut self.elements[self.current]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
