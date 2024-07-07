#[derive(Debug, Clone)]
struct User {
    firstname: String,
    lastname: String,
}

#[warn(clippy::vec_init_then_push)]
fn main() {
    // let mut v = vec![];
    let v = vec![User {
            firstname: "T".to_owned(),
            lastname: "Kumagai".to_owned(),
        },
        User {
            firstname: "S".to_owned(),
            lastname: "Kumagai".to_owned(),
        }
    ];

    for u in v {
        const CHARSET: &str = "abcdefghijklmnopqrstuvwxyz1234567890_!?=<>";
        let vec = CHARSET.chars().collect::<Vec<_>>();

        // TODO: if for contains char
        #[warn(clippy::let_unit_value)]
        let is_charset_contain = vec.iter().for_each(|c| {
            c.eq(&'0').then(|| Some(true).or(Some(false)));
        });
        dbg!(is_charset_contain);

        let v_first_name = u.firstname.chars().collect::<Vec<_>>();
        println!("{:?}", v_first_name);
        let v_last_name = u.lastname.chars().collect::<Vec<_>>();
        println!("{:?}", v_last_name);

        #[warn(clippy::single_char_pattern)]
        if u.firstname.contains('S') {
            println!("{:?}", u);
        }
    }

    println!("{}", gcd(14, 15));
}

fn gcd(mut n: u64, mut m: u64) -> u64 {
    assert!(n != 0 && m != 0);
    while m != 0 {
        if m < n {
            // let t = m;
            #[warn(clippy::manual_swap)]
            std::mem::swap(&mut n, &mut m);
            // m = n;
            // n = t;
        }
        m %= n;
    }
    n
}

#[test]
fn test_gcd() {
    assert_eq!(gcd(14, 15), 1);

    assert_eq!(gcd(2 * 3 * 5 * 11 * 17, 3 * 7 * 11 * 13 * 19), 3 * 11);
}