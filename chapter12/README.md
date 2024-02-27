# 12 演算子のオーバーロード

## 12.1 演算子のオーバーロードまとめ

```rust

std::opt::Neg // -x
std::opt::Not // !x
std::opt::Add // x + y
trait Add<RHS=Self> {
    type Output;
    fn add(self, rhs: RHS) -> Self::Output
}
std::opt::Sub // x - y
```

## 12.2 等価性テスト

```rust

trait PartialEq<Rhs: ?Sized = Self> {
    fn eq(&self, other: &Rhs) -> bool,
    fn ne(&self, other: &Rhs) -> bool { !self.eq(other) }
}
```

## 12.3 順序比較

## 12.4 IndexとIndexMut

std::ops::Indexトレイトとstd::ops::IndexMutトレイトを実装することで、独自の方に対する
a[i]のようなインデックス式の動作を指定することができる。
