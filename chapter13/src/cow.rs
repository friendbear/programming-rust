use std::borrow::Cow;
pub enum Error {
    NotFound,
    PermissionDenied,
}

pub fn cow_sample(e: Error) -> Cow<'static, str> {

    match e {
        Error::NotFound => Cow::Borrowed("Not found"),
        Error::PermissionDenied => Cow::Borrowed("Permission denied"),
        _ => Cow::Borrowed("Unknown error"),
    }
}
pub fn cow_owned_sample() -> Cow<'static, str> {
    let owned_string = String::from("This is an owned string");
    Cow::Owned(owned_string)
}

fn main() {
    let borrowed = cow_sample(Error::NotFound);
    println!("Borrowed: {}", borrowed);

    let owned = cow_owned_sample();
    println!("Owned: {}", owned);
}
