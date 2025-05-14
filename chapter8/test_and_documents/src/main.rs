/// テストとドキュメント

fn main() {
    println!("Hello, world!");
}

#[test]
fn math_works() {
    let x: i32 = 1;
    assert!(!x.is_negative());
    assert!(x + 1 == 2);
}

//#[test]
//#[should_panic(expected = "divide by zero")]
//fn divide_by_zero() {
//    let y: i32 = 0;
//    let _ = 1 / y;
//}

#[test]
#[should_panic(expected = "divide by zero")]
fn divide_by_zero_negative() {
    let y: i32 = -1;
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
