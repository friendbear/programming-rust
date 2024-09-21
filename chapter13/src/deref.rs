//!
//! std::opt::Derefトレイトを実装すると、参照を返すメソッドを実装することができる。
//! これにより、参照を返すメソッドを呼び出すと、参照の中身を取り出すことができる。
//! 参照解決型変換
use std::ops::Deref;
use std::ops::DerefMut;

pub struct Selector<T> {
    pub elements: Vec<T>,
    pub current: usize,
}

impl<T> Deref for Selector<T> {
    type Target = T;
    fn deref(&self) -> &T {
        &self.elements[self.current]
    }
}

impl<T> DerefMut for Selector<T> {
    fn deref_mut(&mut self) -> &mut T {
        &mut self.elements[self.current]
    }
}
