
#[cfg(test)]
mod tests {

    #[test]
    fn test_fully_qualified_01() {
        assert_eq!("hello".to_string(), str::to_string("hello"));
        assert_eq!(ToString::to_string(&"hello"), <str as ToString>::to_string("hello"));
    }
    #[test]
    fn test_fully_qualified_02() {
        let value = 0;
        assert_eq!(value.to_string(), i32::to_string(&value));
    }
}