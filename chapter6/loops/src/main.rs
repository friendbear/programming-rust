use std::error;

fn main() {
    let strings: Vec<String> = error_message();
    for s in &strings {

        println!("String {:?} is at address {:p}.", s, s)
    }
    println!("{} error(s)", strings.len());
}

fn error_message() -> Vec<String> {
    return vec!["400".to_string(), "404".to_string(), "405".to_string(), "500".to_string()];
}