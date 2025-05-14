#[derive(Debug)]
struct Point2D {
    x: i32,
    y: i32,
}

// Rect構造体を定義
#[derive(Debug)]
enum Shape {
    Rect(Point2D, Point2D),
}

fn optimized_paint(shape: &Shape) {
    match shape {
        Shape::Rect(top_left, bottom_right) => {
            println!(
                "Rect top_left: {:?}, bottom_right: {:?}",
                top_left, bottom_right
            );
        }
    }
}

impl Shape {
    fn print(&self) {
        match self {
            rect @ Shape::Rect(..) => {
                // Assuming `optimized_paint` takes a reference to `Shape::Rect`
                // If it takes a `Rect` struct, you would need to construct one here
                optimized_paint(rect);
            }
            // If you add more variants to `Shape`, you would handle them here
            // For now, since `Shape` only has `Rect`, this arm is unreachable
            _ => unreachable!("Shape not handled: {:?}", self),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_debug_print() {
        let top_left = Point2D { x: 1, y: 2 };
        let bottom_right = Point2D { x: 3, y: 4 };
        let rect = Shape::Rect(top_left, bottom_right);
        rect.print();
    }
}
