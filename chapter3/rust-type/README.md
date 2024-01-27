# ３章 基本的な型

## 3.1 機械語型

## 3.2 タプル

## 3.3 ポインタ型

### 3.3.1 参照

```rust
&T // これは変更不能な参照で、Cのconst T*に相当する
&mut T // これは可変参照で、CのT*に相当する
```

### 3.3.2 Box

 ヒープ上に値を確保する最もかんたんな方法は`Box::new`を使う方法

 ```rust
let r = (12, "egg");
let b = Box::new(r); // allocate a tuplue in the heap 
 ```

### 3.3.3 rawポインタ

 Rustには、 `*mut T` と `*const T` というrawポインタが存在する。

## 3.4 配列、ベクタ、スライス

### 3.4.4 スライス

```rust
let v: Vec<f64> = vec![0.0, 0.707, 1.0];
let a: [f64: 3] =     [0.0, 0.707, 1.0];

let sv: &[f64] = &v;
let sa: &[f64] = &a;
```

## 3.5 文字列型

### 3.5.3 メモリ上の文字列型

> ![Tips]
> code

```rust
assert_eq!("".len(), 0);
assert_eq!("000".char().len(), 3);
```

format! マクロ

`format!("{} {:02}{:02}"), 2, 4, 10);`
