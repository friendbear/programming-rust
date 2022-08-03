fn main() {
    println!("Hello, world!");

    // div zero
    let f = || 10 / 0 as i32;

    f(); // panic

    panic!();
}
