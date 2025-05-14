use std::ops::Neg;
pub mod index;

#[derive(Debug, PartialEq, Eq)]
pub struct Complex<T> {
    pub re: T,
    pub im: T,
}

impl<T, O> Neg for Complex<T>
where
    T: Neg<Output = O>,
{
    type Output = Complex<O>;
    fn neg(self) -> Self::Output {
        Complex {
            re: -self.re,
            im: -self.im,
        }
    }
}

trait Not {
    type Output;
    fn not(self) -> Self::Output;
}