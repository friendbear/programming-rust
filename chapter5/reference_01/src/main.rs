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

fn smallest(v: &[i32]) -> &i32 {
    let mut s = &v[0];
    for r in &v[1..] {
        if *r < *s { s = r }
    }
    s
}

#[test]
fn test_smallest() {
    let s;
    {
        let paradola = [9, 4, 1, 0, 1, 4, 9];
        s = smallest(&paradola);
        assert_eq!(*s, 0)
    }
    //asseert_eq!(*s, 0)
}

struct S<'a>{
    r: &'a i32
}

struct T<'a> {
    s: S<'a>
}

#[test]
fn test_struct_in_reference() {

    let s;
    {
        let x = 10;
        s = S { r: &x };
        assert_eq!(*s.r, 10);
    }
}

struct S2<'a, 'b> {
    x: &'a i32,
    y: &'b i32
}

#[test]
fn test_lifetime_two(){
    fn f<'a, 'b>(r: &'a i32, s: &'b i32) -> &'a i32 {
        r
    }
    let a = 10;
    {
        let b = 20;
        assert_eq!(f(&a, &b), &10);
    }
}

struct StringTable {
    elements: Vec<String>,
}
impl StringTable {
    fn find_by_prefix(&self, prefix: &str) -> Option<&String> {
        for i in 0..self.elements.len() {
            if self.elements[i].starts_with(prefix) {
                return Some(&self.elements[i]);
            }
        }
        None
    }
}

fn extend(vec: &mut Vec<f64>, slice: &[f64]) {
    for elt in slice {
        vec.push(*elt)
    }
}

#[test]
fn test_extend() {
    let mut wave = Vec::new();
    let head = vec![1.0, 2.0, 3.0];
    let tail = [0.0, -1.0];

    extend(&mut wave, &head);
    extend(&mut wave, &tail);

    assert_eq!(wave, vec![1.0, 2.0, 3.0, 0.0, -1.0]);
}
