use std::collections::HashMap;
type Table = HashMap<String, Vec<String>>;
fn main() {
    println!("Hello, world!");
    let mut table = Table::new();
    table.insert("Cesualdo".to_string(),
                vec!["Tenebrae Responsoria".to_string(),
                        "many madrigals".to_string()]);
    table.insert("Caravaggio".to_string(),
                vec!["The Musicians".to_string(),
                        "The Calling of St. Matthew".to_string()]);
    table.insert("Cellini".to_string(),
                vec!["Perseus with the head of Medusa".to_string(),
                        "a salt cellar".to_string()]);

    sort_works(&mut table);
    show(&table);
}

fn show(table: &Table) {
    for (artist, works) in table {
        println!("works by {}:", artist);
        for work in works {
            println!("  {}", work)
        }
    }
}

fn sort_works(table: &mut Table) {
    for (_artist, works) in table {
        works.sort()
    }
}

#[test]
fn test_value_reference() {
    let x = 10;
    let r = &x;
    assert_eq!(*r, 10);

    let mut y = 32;
    let m = &mut y;
    *m += 32;
    assert_eq!(*m, 64);

    struct Anime {name: &'static str, bechdel_pass: bool };
    let aria = Anime { name: "Aria The Animation", bechdel_pass: true };
    let anime_ref = &aria;
    assert_eq!(anime_ref.name, "Aria The Animation");
    assert_eq!((*anime_ref).name, "Aria The Animation");
}

#[test]
fn test_refelence1() {
    let x = 10;
    let y = 20;
    let mut r = &x;
    if true { r = &y }

    assert!(*r == 10 || *r == 20);

    let xx = &r;
    let xxx = &xx;
    assert_eq!(***xxx, 20);

    struct Point {x: i32, y: i32 };
    let point = Point {x: 10, y: 20};
    let rr = &&point;
    assert_eq!(rr.x, 10)

}

