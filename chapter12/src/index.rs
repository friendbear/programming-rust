
pub struct Image<P> {
    pub width: usize,
    pub pixels: Vec<P>,
}

impl<P: Default + Copy> Image<P> {
    pub fn new(width: usize, height: usize) -> Image<P> {
        Image {
            width: width,
            pixels: vec![P::default(); width * height],
        }
    }
}

impl <P> std::ops::Index<usize> for Image<P> {
    type Output = [P];
    fn index(&self, row: usize) -> &[P] {
        let start = row * self.width;
        &self.pixels[start .. start + self.width]
    }
}

impl<P> std::ops::IndexMut<usize> for Image<P> {
    fn index_mut(&mut self, row: usize) -> &mut [P] {
        let start = row * self.width;
        &mut self.pixels[start .. start + self.width]
    }
}