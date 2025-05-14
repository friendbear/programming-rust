use std::borrow::Cow;
use chapter13::cow::cow_sample;
use chapter13::cow::cow_owned_sample;
use chapter13::cow::Error;

#[allow(dead_code)]
fn main() {

}
#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_cow_sample() {
        let borrowed = cow_sample(Error::NotFound);
        assert_eq!(borrowed, Cow::Borrowed("Not found"));
        assert_eq!(borrowed, "Not found");
    }

    #[test]
    fn test_cow_owned_sample() {
        let owned = cow_owned_sample();
        //assert_eq!(owned, Cow::Owned(String::from("This is an owned string")));
        assert_eq!(owned, "This is an owned string");
    }
}