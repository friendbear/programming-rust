
//! # Drop
//! `Drop`トレイトは、ある値がスコープを抜けるときに実行される処理を定義するためのトレイトです。
//! 
//! この例では、`Application`構造体が定義されており、`Drop`トレイトを実装しています。
//! `Application`構造体がスコープを抜けるときに、`drop`メソッドが呼び出され、
//! 構造体の`name`フィールドと`nickname`フィールドの内容が出力されます。
//! 
//! ## フィールド
//! - `name`: アプリケーションの名前を表す文字列。
//! - `nickname`: アプリケーションのニックネームを表す文字列のベクター。
//! 
//! ## 例
//! ```rust
//! {
//!     let app = Application {
//!         name: String::from("MyApp"),
//!         nickname: vec![String::from("App"), String::from("My")],
//!     };
//! } // ここで`drop`メソッドが呼び出されます。
//! ```

pub struct Application {
    pub name: String,
    pub nickname: Vec<String>,
}

impl Drop for Application {
    fn drop(&mut self) {
        print!("Dropping {}", self.name);
        if !self.nickname.is_empty() {
            print!("(drop {})", self.nickname.join(", "))
        }
        println!("")
    }
}