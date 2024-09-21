use chapter13::deref::Selector;

fn main() {
    let mut selector = Selector {
        elements: vec!["one", "two", "three"],
        current: 2,
    };

    println!("Current element: {}", *selector);
    *selector = "four";
    println!("Current element: {}", *selector);
}

#[cfg(test)]
mod tests {
    use std::ops::DerefMut;

    use super::*;

    #[test]
    fn test_selector_deref() {
        let mut selector = Selector {
            elements: vec!["one", "two", "three"],
            current: 2,
        };

        let current = *selector;
        assert_eq!(current, "three");
    }

    #[test]
    fn test_selector_deref_mut() {
        let mut selector = Selector {
            elements: vec![1, 2, 3],
            current: 1,
        };

        let current = selector.deref_mut();
        *current = 5;
        assert_eq!(*current, 5);
    }
}