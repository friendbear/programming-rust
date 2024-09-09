use std::collections::HashMap;
use std::fmt::Debug;
use std::hash::Hash;
use std::io::Write;

mod stringset;
/// ジェネリクス関数
/// 
/// # Examples 
fn say_hi<W: Write>(out: &mut W) -> std::io::Result<()> {
    out.write_all(b"hi\n")?;
    out.flush()
}

/// Prints the top ten items in a list.
pub fn top_ten<T>(list: &Vec<T>)
where
    T: Eq + Hash + Debug,
{
    let mut map = HashMap::new();
    for item in list {
        let count = map.entry(item).or_insert(0);
        *count += 1;
    }
    let mut pairs: Vec<_> = map.into_iter().collect();
    pairs.sort_by(|&(_, a), &(_, b)| b.cmp(&a));
    for (item, count) in pairs.into_iter().take(10) {
        println!("{:?}: {}", item, count);
    }
}

/// トレイトでのSelf
#[derive(Debug)]
struct CherryTree {
    height: u32,
    fruit: u32,
}
#[derive(Debug)]
struct Mammoth{ weight: u32, height: u32 }

pub trait Spliceable {
    fn splice(&self, other: &Self) -> Self;
}

impl Spliceable for CherryTree {
    fn splice(&self, other: &Self) -> Self {
        CherryTree {
            height: (self.height + other.height) / 2,
            fruit: self.fruit + other.fruit,
        }
    }
}

impl Spliceable for Mammoth {
    fn splice(&self, other: &Self) -> Self {
        Mammoth {
            weight: (self.weight + other.weight) / 2,
            height: (self.height + other.height) / 2,
        }
    }
}

/// サブトレイト
pub trait SpliceableExt: Spliceable {
    fn splice_and_print(&self, other: &Self) -> Self
    where Self: Debug + Sized
    {
        let result = self.splice(other);
        println!("Spliced: {:?}", result);
        result
    }
}

impl SpliceableExt for CherryTree {}
impl SpliceableExt for Mammoth {}



#[cfg(test)]
/// Tests the behavior of the `say_hi` function.
///
/// This test function checks the behavior of the `say_hi` function by performing the following steps:
///
/// 1. Creates an empty vector `bytes`.
/// 2. Calls the `say_hi` function with the mutable reference to `bytes`.
/// 3. Asserts that the `bytes` vector is equal to the byte string `b"hi\n"`.
///
/// 4. Creates a new `Cursor` object `c` with an empty vector.
/// 5. Calls the `say_hi` function with the mutable reference to `c`.
/// 6. Asserts that the inner contents of `c` (obtained by calling `into_inner()`) is equal to the byte string `b"hi\n"`.
///
/// This test ensures that the `say_hi` function correctly writes the byte string `b"hi\n"` to both a vector and a `Cursor` object.
mod test {
    use super::*;
    use std::io::Cursor;

    #[test]
    fn test_say_hi() {
        let mut bytes = vec![];
        say_hi(&mut bytes).unwrap();
        assert_eq!(bytes, b"hi\n");

        
        let mut c = Cursor::new(vec![]);
        say_hi(&mut c).unwrap();
        assert_eq!(c.into_inner(), b"hi\n");
    }

    #[test]
    fn test_top_ten() {
        let v = vec![1, 2, 3, 4, 5, 1, 2, 3, 4, 5, 1, 2, 3, 4, 5];
        top_ten(&v);
    }

    #[test]
    fn test_splice() {
        let cherry1 = CherryTree {
            height: 10,
            fruit: 100,
        };
        let cherry2 = CherryTree {
            height: 20,
            fruit: 200,
        };
        let cherry3 = cherry1.splice(&cherry2);
        assert_eq!(cherry3.height, 15);
        assert_eq!(cherry3.fruit, 300);

        let mammoth1 = Mammoth {
            weight: 1000,
            height: 300,
        };
        let mammoth2 = Mammoth {
            weight: 2000,
            height: 400,
        };
        let mammoth3 = mammoth1.splice(&mammoth2);
        assert_eq!(mammoth3.weight, 1500);
        assert_eq!(mammoth3.height, 350);
    }

    #[test]
    fn test_splice_and_print() {
        let cherry1 = CherryTree {
            height: 10,
            fruit: 100,
        };
        let cherry2 = CherryTree {
            height: 20,
            fruit: 200,
        };
        let cherry3 = cherry1.splice_and_print(&cherry2);

        assert_eq!(cherry3.height, 15);

        let mammoth1 = Mammoth {
            weight: 1000,
            height: 300,
        };
        let mammoth2 = Mammoth {
            weight: 2000,
            height: 400,
        };
        let mammoth3 = mammoth1.splice_and_print(&mammoth2);
        assert_eq!(mammoth3.weight, 1500);
    }

}