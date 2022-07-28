fn main() {
    println!("Hello, world!");
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

