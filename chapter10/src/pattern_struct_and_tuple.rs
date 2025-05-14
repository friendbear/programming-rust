/// # Tuple and Struct Pattern
/// One of the most common uses of pattern matching is to destructure a struct or enum. This is
/// done by matching the struct or enum with a pattern that describes its fields. The fields can
/// then be used in the match arm.
pub fn describe_point(x: i32, y: i32) -> &'static str {
    use std::cmp::Ordering::*;
    match (x.cmp(&0), y.cmp(&0)) {
        (Equal, Equal) => "at the origin",
        (_, Equal) => "on the x axiis",
        (Equal, _) => "on the y axis",
        (Greater, Greater) => "in the first quadrant",
        (Less, Greater) => "in the second quadrant",
        _ => "somewhere else",
    }
}

pub struct Point {
    x: i32,
    y: i32,
}

pub fn describe_point_struct(point: Point) -> &'static str {
    use std::cmp::Ordering::*;
    match (point.x.cmp(&0), point.y.cmp(&0)) {
        (Equal, Equal) => "at the origin",
        (_, Equal) => "on the x axiis",
        (Equal, _) => "on the y axis",
        (Greater, Greater) => "in the first quadrant",
        (Less, Greater) => "in the second quadrant",
        _ => "somewhere else",
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_describe_point() {
        assert_eq!(super::describe_point(0, 0), "at the origin");
        assert_eq!(super::describe_point(0, 7), "on the y axis");
        assert_eq!(super::describe_point(7, 0), "on the x axiis");
        assert_eq!(super::describe_point(7, 7), "in the first quadrant");
        assert_eq!(super::describe_point(-7, 7), "in the second quadrant");
        assert_eq!(super::describe_point(-7, -7), "somewhere else");
    }

    #[test]
    fn test_describle_point_struct() {
        let point = Point { x: 10, y: 20 };
        assert_eq!(super::describe_point_struct(point), "in the first quadrant");
    }
}
