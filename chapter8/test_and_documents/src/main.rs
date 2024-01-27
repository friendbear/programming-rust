fn main() {
    println!("Hello, world!");
}

#[test]
fn math_works() {
    let x: i32 = 1;
    assert!(!x.is_negative());
    assert!(x + 1 == 2);
}

/// panic test
#[test]
#[should_panic(expected = "divide by zero")]
fn divide_by_zero() {
    let y = 0;
    let _ = 1 / y;
}

#[cfg(test)]
mod tests {
    fn roughly_equal(a: f64, b: f64) -> bool {
        (a - b).abs() < 1e-6
    }

    #[test]
    fn trig_works() {
        use std::f64::consts::PI;
        assert!(roughly_equal(PI.sin(), 0.0));
    }
}
