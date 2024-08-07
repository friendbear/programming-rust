use chapter12::Complex;
use std::ops::Neg;

fn main() {
    let x_complex = Complex::<i32> { re: 18, im: 200 };
    let y_complex = Complex::<i32> { re: -18, im: -200 };
    //    let x_complex_neg = x_complex.neg();

    assert_eq!(x_complex.neg(), y_complex);
}
