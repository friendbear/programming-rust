use std::ops::{RangeTo, Range};

#[derive(Copy, Clone)]
struct Label { number: u32 }

/// Copy traitを実装できない、 nameフィールドは Copyを実装していない
//#[derive(Copy, Clone)]
//struct StringLabel{ name: String }

fn print(l: Label) {
    println!("STAMP: {}", l.number);
}

fn main() {
    let s = vec!["udon".to_string(), "ramen".to_string(), "soba".to_string()];
    let t = s; // value moved here
    //let u = s;  // value used here after move

    // 明示的にコピーがほしいことを要求する .clone();
    let s1 = vec!["udon".to_string(), "ramen".to_string(), "soba".to_string()];
    let _t1 = s1.clone();
    let _u1 = s1.clone();

    let mut _s = "Govinda".to_string();
    _s = "Siddharta".to_string(); // value "Govinda" dropped here

    move_and_flow_controll();

    let str_v = create_string_vec(Range{ start: 101, end: 1005});
    let filter: Vec<&String> = str_v.iter().filter(|x| x.ends_with("5")).collect();
    println!("{:?}", filter);

    // Copy Type
    let l = Label { number: 5};
    print(l);
    println!("STAMP: {}", l.number);


}
#[test]
fn test_move_string() {

    let mut str_101_to_105 = create_string_vec(Range{ start: 101, end: 105});

    assert_eq!(str_101_to_105.len(), 5);
    // 1. Pop avalue off the end of the vector:
    let fifth = str_101_to_105.pop().unwrap();
    assert_eq!(fifth, "105");

    // 2. Move a value out of the middle of the vector,
    let second = str_101_to_105.swap_remove(1);
    assert_eq!(second, "102");

    // 3. Swap in another value for the one we're taking out:
    let third = std::mem::replace(&mut str_101_to_105[2], "substitute".to_string());
    assert_eq!(third, "103");
}

fn move_and_flow_controll() {
    let c = false;
    let x = vec![10, 20, 30];
    if c {
        f(x);
    } else {
        g(x)
    }
    // h(x) // bad
    fn f(v: Vec<i32>) {
        for d in &v {
            println!("fn v{:?}", d);
        }
    }
    fn g(v: Vec<i32>) {
        println!("fn g{:?}", v);
    }
}

fn _move_and_flow_contloll2() {
    let mut x = vec![10, 20, 30];
    while f() {
        g(x);
        x = h();
    }

    fn f() -> bool { false }
    fn g(_v: Vec<i32>) {todo!();}
    fn h() -> Vec<i32> {todo!();}
}

fn create_string_vec(range: Range<u32>) -> Vec<String> {
    let mut v = Vec::new();
    for i in range.start..range.end+1 {
        v.push(i.to_string())
    }
    v
}
