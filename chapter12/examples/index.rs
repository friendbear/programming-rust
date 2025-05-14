use chapter12::Image;

fn main() {
    let image = Image::<i32> {
        width: 4,
        pixels: vec![1, 2, 3, 4],
    };

    dbg!(&image);

    assert_eq!(image[0][2], 3);
}
