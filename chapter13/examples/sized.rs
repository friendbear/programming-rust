use chapter13::sized::RcBox;
use chapter13::sized::S;

use std::fmt::Display;
fn main() {

    // RcBox<Display> is not Sized
    // let boxed_lunch: RcBox<Display> = RcBox { // error[E0277]: the trait bound `Display: std::marker::Sized` is not satisfied
    //     ref_count: 1,
    //     value: "lunch".to_string(),
    // };
    // let boxed_displayable: &RcBox<Display> = &boxed_lunch;
    // display(boxed_displayable);

    let boxed_lunch: RcBox<String> = RcBox {
        ref_count: 1,
        value: "lunch".to_string(),
    };

    let boxed_displayable: &RcBox<dyn Display> = &boxed_lunch;
    display(boxed_displayable);

    let s = S {
        b: Box::new("hello"),
    };
    println!("{}", s.b);
}
fn display(boxed: &RcBox<dyn Display>) {
    println!("For your enjoyment: {}", &boxed.value);
}
