# 7. エラー処理

## 7.1 パニック

* 配列への範囲外へのアクセス
* 整列のゼロによる除算
* NoneであるOptionに対する、.unwrap()
* アサートの失敗

### program中

panic!() マクロ呼び出し

パニックはスレッド単位に発生する。

## 7.2 Result

### 7.2.1 エラーの補足

### 7.2.2 Result型のエイリアス

`type Result<T> = Result<T, Error>`

### 7.2.3 エラーの表示

### 7.2.4 エラーの伝搬

### 7.2.5 複数種類エラーへの対応

> [!IMPORTANT]
> `Box::<std::error::Error>`
> 
> すべてのエラーを表すタイプ

### 7.2.6 「起こるはずがない」エラーの処理

### 7.2.7 エラーを無視する

```rust
let _ = writeln!(stderr(), "error:()", err); // ok
```

### 7.2.8 main()でのエラー処理

```rust
fn main() {
    calculate_tides().expect("error"); // the buck stops here
}
```

```rust
fn main() {
    if let Err(err) = calculate_tides() {
        std::process::exit(1);
    }
}
```
