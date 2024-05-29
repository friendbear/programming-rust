use std::rc::Rc;

fn main() {
    let s: Rc<String> = Rc::new("shirataki".to_string());
    let t: Rc<String> = s.clone();
    let u: Rc<String> = s.clone();
}

#[test]
fn test_owner_share() {
    let s: Rc<String> = Rc::new("shirataki".to_string());
    {

        let t = s.clone();
        {

            let u = s.clone();
            println!("{} are quite chewy, almost bouncy, but lack flavor", u);
            assert_eq!(Rc::strong_count(&s), 3);
        }
        assert_eq!(t.find("taki"), Some(5));
        assert_eq!(Rc::strong_count(&s), 2);

    }
    assert!(s.contains("shira"));
    assert_eq!(Rc::strong_count(&s), 1);
}
