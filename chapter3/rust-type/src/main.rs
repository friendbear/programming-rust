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
}