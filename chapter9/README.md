# 構造体

Rustの３つの構造体は、それぞれ以下のようになっています。
名前空間の構造体は、Rustのプログラムやライブラリを構成する関数、型、定数などを収めるコンテナである。
名前付きフィールド型、タプル型、ユニット型の３つがあります。

## 名前付きフィールド型

```rust
/// A rectangle of eight-bit grayscale pixels. // 8ビットグレースケールピクセルの長方形
struct GrayscaleMap {
    pixels: Vec<u8>, // ピクセル
    size: (usize, usize), // サイズ
}
```

## 構造体式

```rust
let width = 1024;
let height = 576;
let image = GrayscaleMap {
    pixels: vec![0; width * height],
    size: (width, height),
};
```

* New Map

```rust
fn new_map(width: usize, height: usize) -> GrayscaleMap {
    GrayscaleMap {
        pixels: vec![0; width * height],
        size: (width, height),
    }
}
```

## Implによるメソッドの定義

```rust
impl GrayscaleMap {
    fn get_pixel(&self, x: usize, y: usize) -> Option<&u8> {
        self.pixels.get(x * self.size.0 + y)
    }
}
```

## ジェネリクス構造体

```rust
struct Broom<T> {
    x: T,
    y: T,
}
```

## 生存期間パラメータを持つ構造体

構造体が参照を含むのであれば、参照の生存期間を
指定する必要があります。

```rust
struct Broom<'a, T> {
    x: &'a T,
    y: T,
}
```

## 一般的なトレイトの自動実装

```rust
#[derive(Copy, Clone, Debug, PartialEq)]
struct Point {
    x: f64
    y: f64,
}
```

## 内部可変性

Rustの構造体は、内部可変性を持つことができます。
内部可変性とは、構造体のフィールドを変更するために
可変参照を取得することです。

```rust
use std::cell::Cell;

struct CellPoint {
    x: f64,
    y: Cell<f64>,
}

let point = CellPoint {
    x: 1.0,
    y: Cell::new(2.0),
};
point.y.set(3.0);

assert_eq!(point.x, 1.0);
assert_eq!(point.y.get(), 3.0);
```

RefCellとCellの違いは、RefCellは実行時に借用規則をチェックするが、Cellはコンパイル時にチェックする。

```rust
use std::cell::RefCell;

struct RefCellPoint {
    x: f64,
    y: RefCell<f64>,
}

let point = RefCellPoint {
    x: 1.0,
    y: RefCell::new(2.0),
};

*point.y.borrow_mut() = 3.0;
*point.y.borrow() = 4.0;

```
