use std::{error, io::Read};

fn main() {
    let strings: Vec<String> = error_message();
    for s in &strings {
        println!("String {:?} is at address {:p}.", s, s)
    }
    println!("{} error(s)", strings.len());

    let stdin = read_vec::<String>();

    for line in read_vec::<String>() {
        if line.is_empty() {
            continue;
        }
        println!("stdin: {:?}", line)
    }
}

fn _read<T: std::str::FromStr>() -> T {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).ok();
    s.trim().parse().ok().unwrap()
}

fn read_vec<T: std::str::FromStr>() -> Vec<T> {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).ok();
    s.split_whitespace()
        .map(|e| e.parse().ok().unwrap())
        .collect()
}

fn error_message() -> Vec<String> {
    return vec![
        "400".to_string(),
        "404".to_string(),
        "405".to_string(),
        "500".to_string(),
    ];
}
