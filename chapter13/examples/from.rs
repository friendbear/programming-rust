use chapter13::from::string_into;
fn main() {

}

#[cfg(test)]

mod tests {
    use super::*;

    #[test]
    fn test_string_into() {
        let result = string_into(&"hello".to_owned());
        assert_eq!(result, String::from(r#""hello""#).into_bytes());
    }
}