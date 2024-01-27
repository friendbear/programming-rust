struct Extrema<'elt> {
    greatest: &'elt i32,
    latest: &'elt i32,
}

fn find_extrema<'s>(slice: &'s [i32]) -> Extrema<'s> {
    let mut greatest = &slice[0];
    let mut latest = &slice[0];
    for i in 1..slice.len() {
        if slice[i] < *latest {
            latest = &slice[i];
        }
        if slice[i] > *greatest {
            greatest = &slice[i];
        }
    }
    Extrema { greatest, latest }
}

#[test]
fn test_extrema() {
    let a = [0, -3, 0, 15, 48];
    let e = find_extrema(&a);
    assert_eq!(*e.latest, -3);
    assert_eq!(*e.greatest, 48);
}
