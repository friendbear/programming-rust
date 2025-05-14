//! Sized
pub struct S<T:?Sized> {
    pub b: Box<T>,
}

pub struct RcBox<T: ?Sized> {
    pub ref_count: usize,
    pub value: T,
}