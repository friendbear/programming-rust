#[derive(Copy, Clone, Debug, PartialEq)]
struct Point {
    x: f64,
    y: f64,
}
fn main() {
    let p1 = Point { x: 1.0, y: 2.0 };
    println!("{x:?}, {y:?}", x = p1.x, y = p1.y);
}
