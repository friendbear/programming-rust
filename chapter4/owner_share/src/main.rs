use std::rc::Rc;
use std::sync::{Arc, Mutex};
use std::thread;

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


#[test]
fn test_owner_share_arc() {
    let s: Arc<Mutex<String>> = Arc::new(Mutex::new("shirataki".to_string()));
    
    {
        let t = s.clone();
        {
            let u = s.clone();
            let u = u.lock().unwrap();
            println!("{} are quite chewy, almost bouncy, but lack flavor", u);
            assert_eq!(Arc::strong_count(&s), 3);
        }
        let t = t.lock().unwrap();
        assert_eq!(t.find("taki"), Some(5));
        assert_eq!(Arc::strong_count(&s), 2);
    }
    let s_clone = Arc::clone(&s);
    let s_guard = s.lock().unwrap();
    assert!(s_guard.contains("shira"));

    let handle = thread::spawn(move || {
        let sparkle_heart = vec![240, 159, 146, 150];

        let sparkle_heart = String::from_utf8(sparkle_heart).unwrap();

        let mut s = s_clone.lock().unwrap();
        s.push_str(String::as_str(&sparkle_heart));
        assert_eq!("💖", sparkle_heart);
    });
    handle.join().unwrap();
    s_guard.contains("💖");
    drop(s_guard);

}