# 6. 式

## 6.1 式言語

## 6.2 ブロックとセミコロン

## 6.3 宣言

## 6.4 ifとmatch

### 6.4.1 if let式

## 6.5 ループ

```rust
std::opt::Range {
    start: 0, end: 20
}
```

## 6.6 return式

## 6.7 なぜRustにはloop式があるのか

> [!INPORTANT]
> ! が意味するのは、exit()は決して帰ってこない関数であるという意味だ。

```rust
fn std::process::exit() -> !
```

## 6.8 関数呼び出しとメソッド呼び出し

## 6.9 フィールドと要素

配列から要素の取り出し（スライス）

## 6.10 参照演算子「&」

## 6.11 算術演算子、ビット演算子、比較演算子、論理演算子

## 6.12 論理演算子

## 6.13 型キャスト

> [!Tips]
> 自動変換

* 型&Stringの型は、&str型にキャスト無しでキャストされる
* 型&Vec<i32>の値は&[i32]に自動変換される
* &Box<Chessboard> の値は、&Chessboardに自動変換される

## 6.14 クロージャ

軽量な関数のような値、Closure

```rust
let sum = |x, y| -> x + y;
```
