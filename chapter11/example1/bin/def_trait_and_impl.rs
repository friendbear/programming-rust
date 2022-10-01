trait Visible {
    /// Render this object on the given canvas.
    fn draw(&self, canvas: &mut Canvas);

    /// Return true if clicking at (x, y) should
    /// select this object
    fn hit_test(&self, x: i32, y: i32) -> bool;
}

impl Visible for Broom {
    fn draw(&self, canvas: &mut Canvas) {
        for y in self.y - self.height -1 .. self.y {
            canvas.write_at(self.x, y, '|');
            canvas.write_at(self.x, self.y, 'M');
        }
    }

    fn hit_test(&self, x: i32, y: i32) -> bool {
        self.x == x && self.y - self.height -1 <= y
        && y <= self.y
    }
}

impl Broom {
    /// Helper function used by Broom::drow() below.
    fn broomstick_range(&self) -> Range<i32> {
        self.y - self.height -1 .. self.y
    }

}
fn main() {

}