#[derive(Debug, Clone)]
struct User {
    firstname: String,
    lastname: String,
}


fn main() {

    let v = vec![
        User{ firstname: "T".to_owned(),  lastname: "Kumagai".to_owned()},
        User{ firstname: "S".to_owned(),  lastname: "Kumagai".to_owned()},
    ];

    for u in v {
        const CHARSET: &str = "abcdefghijklmnopqrstuvwxyz1234567890_!?=<>";
        let vec = CHARSET.chars().collect::<Vec<_>>();
        if u.firstname.contains("S") {

            println!("{:?}", u);
        }
    }

    println!("{}", gcd(14, 15));
}

fn gcd(mut n: u64, mut m: u64) -> u64  {
    assert!(n != 0 && m != 0);
    while m != 0 {
        if m < n {
            let t = m;
            m = n;
            n = t;
        }
        m = m % n;
    }
    n
}

#[test]
fn test_gcd() {
    assert_eq!(gcd(14, 15), 1);

    assert_eq!(gcd(2 * 3 * 5 * 11 * 17,
                   3 * 7 * 11 * 13 * 19),
               3 * 11);
}