use std::ops::{Index, IndexMut};
use chapter12::index::Image;

fn main() {
    let image = Image::<i32> {
        width: 4,
        pixels: vec![1, 2, 3, 4],
    };

    println!("{:?}", image[0][2]);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_image() {
        let image = Image::<i32> {
            width: 4,
            pixels: vec![1, 2, 3, 4],
        };

        assert_eq!(image[0][2], 3);
    }

    #[test]
    fn test_image_mut() {
        let mut image = Image::<i32> {
            width: 4,
            pixels: vec![1, 2, 3, 4],
        };

        image[0][2] = 5;
        assert_eq!(image[0][2], 5);
    }

    #[test]
    fn test_image_new() {
        let image = Image::<i32>::new(4, 4);
        assert_eq!(image[0][0], 0);
    }

}


