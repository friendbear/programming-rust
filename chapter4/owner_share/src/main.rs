use std::rc::Rc;

fn main() {
    let s: Rc<String> = Rc::new("shirataki".to_string());
    let t: Rc<String> = s.clone();
    let u: Rc<String> = s.clone();

}

#[test]
fn test_owner_share() {
    let s: Rc<String> = Rc::new("shirataki".to_string());
    let t: Rc<String> = s.clone();
    let u: Rc<String> = s.clone();

    assert!(s.contains("shira"));
    assert_eq!(t.find("taki"), Some(5));
    println!("{} are quite chewy, almost bouncy, but lack flavor", u);
}
