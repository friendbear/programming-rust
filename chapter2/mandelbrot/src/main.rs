extern crate num;

use num::Complex;

fn main() {
    println!("Hello, world!");
}

fn complex_squre_add_loop(c: Complex<f64>) {
    let mut z = Complex { re: 0.0, im: 0.0 };
    loop {
        z = z * z + c;
    }
}

fn escape_time(c: Complex<f64>, limit: u32) -> Option<u32> {
    let mut z = Complex {re: 0.0, im: 0.0 };
    for i in 0..limit {
        z = z * z + c;
        if z.norm_sqr() > 4.0 {
            return Some(i);
        }
    }
    None
}

use std::str::FromStr;

fn parse_pair<T: FromStr>(s: &str, separator: char) -> Option<(T, T)> {
    match s.find(separator) {
        None => None,
        Some(index) => {
            match (T::from_str(&s[..index]), T::from_str(&s[index + 1..])) {
                (Ok(l), Ok(r)) => Some((l, r)),
                _ => None

            }
        }
    }
}

#[test]
fn test_parse_pair() {
    assert_eq!(parse_pair::<i32>("",            ','), None);
    assert_eq!(parse_pair::<i32>("1,1",         ','), Some((1, 1)));
    assert_eq!(parse_pair::<f64>("75.1:9.0",    ':'), Some((75.1, 9.0)));
    assert_eq!(parse_pair::<u32>("1,1",         ','), Some((1, 1)));
    assert_eq!(parse_pair::<u32>("1:1",         ','), None);
}