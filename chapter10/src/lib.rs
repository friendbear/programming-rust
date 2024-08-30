mod btree;
mod pattern;
mod pattern_at;
mod pattern_reference;
mod pattern_struct_and_tuple;

/// HTTPステータスコードを表す列挙型
pub enum HttpStatus {
    Ok = 200,
    NotModified = 304,
    NotFound = 404,
}

/// 時間単位を表す列挙型
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TimeUnit {
    Seconds,
    Minutes,
    Hours,
    Days,
    Months,
    Years,
}

/// 単位に関するメソッドを提供する
impl TimeUnit {
    pub fn plural(self) -> &'static str {
        match self {
            Self::Seconds => "seconds",
            Self::Minutes => "minutes",
            Self::Hours => "hours",
            Self::Days => "days",
            Self::Months => "months",
            Self::Years => "years",
        }
    }
    pub fn singular(self) -> &'static str {
        self.plural().trim_end_matches('s')
    }
}

/// 粗い時間を表す列挙型
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum RoughTime {
    InThePast(TimeUnit, u32),
    JustNow,
    InTheFuture(TimeUnit, u32),
}

pub struct Point3d {
    x: f32,
    y: f32,
    z: f32,
}
/// 構造体型列挙型
pub enum Shape {
    Sphere { center: Point3d, radius: f32 },
    Cuboid { corner1: Point3d, corner2: Point3d },
}

impl Shape {
    pub fn volume(&self) -> f32 {
        match self {
            Self::Sphere { radius, .. } => 4.0 / 3.0 * std::f32::consts::PI * radius.powi(3),
            Self::Cuboid { corner1, corner2 } => {
                let diff = Point3d {
                    x: (corner1.x - corner2.x).abs(),
                    y: (corner1.y - corner2.y).abs(),
                    z: (corner1.z - corner2.z).abs(),
                };
                diff.x * diff.y * diff.z
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::mem;
    #[test]
    fn test_http_status_size_of() {
        let size = mem::size_of::<HttpStatus>();
        assert_eq!(size, 2);
        assert_eq!(mem::size_of::<HttpStatus>(), 2);
        assert_eq!(HttpStatus::NotFound as i32, 404);
    }
    #[test]
    fn test_time_unit() {
        assert_eq!(TimeUnit::Seconds.plural(), "seconds");
        assert_eq!(TimeUnit::Seconds.singular(), "second");
        assert_eq!(TimeUnit::Minutes.plural(), "minutes");
        assert_eq!(TimeUnit::Minutes.singular(), "minute");
        assert_eq!(TimeUnit::Hours.plural(), "hours");
        assert_eq!(TimeUnit::Hours.singular(), "hour");
        assert_eq!(TimeUnit::Days.plural(), "days");
        assert_eq!(TimeUnit::Days.singular(), "day");
        assert_eq!(TimeUnit::Months.plural(), "months");
        assert_eq!(TimeUnit::Months.singular(), "month");
        assert_eq!(TimeUnit::Years.plural(), "years");
        assert_eq!(TimeUnit::Years.singular(), "year");
    }

    #[test]
    fn test_rough_time() {
        let r = RoughTime::InThePast(TimeUnit::Seconds, 7);
        assert_eq!(r, RoughTime::InThePast(TimeUnit::Seconds, 7));
        assert_ne!(r, RoughTime::InThePast(TimeUnit::Seconds, 8));
        assert_ne!(r, RoughTime::InThePast(TimeUnit::Minutes, 7));
        assert_ne!(r, RoughTime::JustNow);
        assert_ne!(r, RoughTime::InTheFuture(TimeUnit::Seconds, 7));
    }

    #[test]
    fn test_shape() {
        let c = Point3d {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        };
        let s = Shape::Sphere {
            center: c,
            radius: 1.0,
        };
        match s {
            Shape::Sphere { center, radius } => {
                assert_eq!(center.x, 0.0);
                assert_eq!(center.y, 0.0);
                assert_eq!(center.z, 0.0);
                assert_eq!(radius, 1.0);
            }
            Shape::Cuboid { .. } => {
                panic!("Expected a sphere, got a cuboid!");
            }
        }
        let corner1 = Point3d {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        };
        let corner2 = Point3d {
            x: 1.0,
            y: 1.0,
            z: 1.0,
        };
        let c = Shape::Cuboid { corner1, corner2 };
        assert_eq!(c.volume(), 1.0);
        match c {
            Shape::Cuboid { corner1, corner2 } => {
                assert_eq!(corner1.x, 0.0);
                assert_eq!(corner1.y, 0.0);
                assert_eq!(corner1.z, 0.0);
                assert_eq!(corner2.x, 1.0);
                assert_eq!(corner2.y, 1.0);
                assert_eq!(corner2.z, 1.0);
            }
            Shape::Sphere { .. } => {
                panic!("Expected a cuboid, got a sphere!");
            }
        }
    }
}
