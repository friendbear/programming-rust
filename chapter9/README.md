# 構造体

Rustの３つの構造体は、それぞれ以下のようになっています。
名前空間の構造体は、Rustのプログラムやライブラリを構成する関数、型、定数などを収めるコンテナである。
名前付きフィールド型、タプル型、ユニット型の３つがあります。

# 名前付きフィールド型

```rust
/// A rectangle of eight-bit grayscale pixels. // 8ビットグレースケールピクセルの長方形
struct GrayscaleMap {
    pixels: Vec<u8>, // ピクセル
    size: (usize, usize), // サイズ
}
```

構造体式
```rust
let width = 1024;
let height = 576;
let image = GrayscaleMap {
    pixels: vec![0; width * height],
    size: (width, height),
};
```

New Map
```rust
fn new_map(width: usize, height: usize) -> GrayscaleMap {
    GrayscaleMap {
        pixels: vec![0; width * height],
        size: (width, height),
    }
}
```
