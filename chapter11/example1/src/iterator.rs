use std::env::{args, Args};
use std::fmt::Debug;

pub trait MyIterator {
    type Item;
    fn next(&mut self) -> Option<Self::Item>;
    fn dump(&mut self) 
        where Self: MyIterator<Item = String>  
    {
        todo!()
    }
}

impl MyIterator for Args {
    type Item = String;
    fn next(&mut self) -> Option<Self::Item> {
        if args().len() > 0 {
            Some(<Args as Iterator>::next(&mut *self).unwrap())
        } else {
            None
        }
    }
}

fn collect_into_vec<I: MyIterator>(iter: &mut I) -> Vec<I::Item> {
    let mut vec = Vec::new();
    while let Some(item) = iter.next() {
        vec.push(item)
    }
    vec
}
fn dump(iter: &mut dyn Iterator<Item = String>) {
    for item in iter {
        println!("{:?}", item);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[ignore]
    fn test_args() {
        let mut args = args();
        assert_eq!(<Args as Iterator>::next(&mut args).unwrap(), "target/debug/chapter11-example1");
    }
}