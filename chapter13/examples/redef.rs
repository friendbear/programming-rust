use chapter13::Selector;
fn main() {
    let mut s = Selector {
        elements: vec!['x', 'y', 'z'],
        current: 2,
    };

    assert_eq!(*s, 'z');
    assert!(s.is_alphabetic());

    *s = 'w';

    assert_eq!(s.elements, ['x', 'y', 'w']);
}
