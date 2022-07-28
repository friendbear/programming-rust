use regex::Regex;


fn main() {
    println!("Hello, world!");

    slice();
    
    // raw string
    println!(r###"
        This raw string start with 'r###"'.
        Therefore is does not end until we reach a quote mark ('"'")
        followed immediately by three pound signs ('###'):
    "###);
}

#[test]
fn test_basic_type(){
    assert_eq!(5f32.sqrt() * 5f32.sqrt(), 5.);
    assert_eq!((-1.01f64).floor(), -2.0);
    assert!((-1. / std::f32::INFINITY).is_sign_negative());

    assert_eq!((2.0_f64).sqrt(), f64::sqrt(2.0));

    /// 真偽値型
    assert_eq!(false as i32, 0);
    assert_eq!(true as i32, 1);

}

#[test]
fn test_tuple() {
    let text = "I see the eigenvalue in thine eye";
    let (head, tail) = text.split_at(21);
    assert_eq!(head, "I see the eigenvalue ");
    assert_eq!(tail, "in thine eye");
}

#[test]
fn test_array() {
    let caterrer: [u32; 6] = [1, 2, 4, 7, 11, 16];
    let taxonomy = ["Animalia", "Arthrpopoda", "Insecta"];

    assert_eq!(caterrer[3], 7);
    assert_eq!(taxonomy.len(), 3);

    let zero_array: [u8; 1024] = [0u8; 1024];
    assert_eq!(zero_array[0], 0);

    let mut chaos = [3, 5, 4, 1, 2];
    chaos.sort();
    assert_eq!(chaos, [1, 2, 3, 4, 5]);
}

#[test]
fn test_vec() {
    let mut v = vec![2, 3, 5, 7];

    v.push(1);
    v.push(1);
    v.push(1);
    assert_eq!(v.iter().fold(1, |a, b| a * b), 210);


    fn new_pixel_buffer(row: usize, cols: usize) -> Vec<u8> {
        vec![0; row * cols]
    }

    assert_eq!(new_pixel_buffer(10, 10).len(), 100);


    let mut v = Vec::new();
    v.push("step");
    v.push("on");
    v.push("no");
    v.push("pets");

    assert_eq!(v, vec!["step", "on", "no", "pets"]);

    let v: Vec<i32> = (0..5).collect();
    assert_eq!(v, vec![0, 1, 2, 3, 4]);

    // with_capacity
    let mut v = Vec::with_capacity(2);
    assert_eq!(v.len(), 0);
    assert_eq!(v.capacity(), 2);

    v.push(1);
    v.push(1);
    assert_eq!(v.len(), 2);
    assert_eq!(v.capacity(), 2);

    v.push(2);
    assert_eq!(v.len(), 3);
    assert_eq!(v.capacity(), 4);

    let mut v = vec![10, 20, 30, 40, 50];
    v.insert(3, 35);
    assert_eq!(v, [10, 20, 30, 35, 40, 50]);

    v.remove(1);
    assert_eq!(v, [10, 30, 35, 40, 50]);

    // pop
    let mut v = vec!["carmen", "miranda"];
    assert_eq!(v.pop(), Some("miranda"));
    assert_eq!(v.pop(), Some("carmen"));
    assert_eq!(v.pop(), None);

}

fn slice() {
    let v = vec![0.0, 0.707, 1.0, 0.707];
    let a: [f64; 4] =     [0.0, 0.707, 1.0, 0.707];

    let sa: &[f64] = &v;
    let va: &[f64] = &a;

    print(&v);
    print(&a);
    print(&sa);
    print(&va);
    print(&v[0..2]);
    print(&a[2..]);
    print(&sa[1..3]);
}
fn print(n: &[f64]) {
    for elt in n {
        println!("{}", elt);
    }
}

#[test]
fn test_string() {
    let _speech = "Ouch! said the well.\n";

    println!("In the room the women come and go, 
                Singing of Mount Abora");

    assert_eq!("It was a bright, cold day in April, and \
            there were four of us-\
            more or less.", 
            "It was a bright, cold day in April, and there were four of us-more or less.");

    // raw string
    let _default_win_install_path = r"c:\Program Files";

    let pattern = Regex::new(r"\d+(.\.d+)*").unwrap();
    assert_eq!(pattern.is_match("123-4567"), true);

}

#[test]
fn test_byte_str() {
    let method = b"GET";
    assert_eq!(method, &[b'G', b'E', b'T']);
}