
/// 与えられた引数を `AsRef<str>` トレイトを使用して文字列スライスに変換し、
/// その結果を `print_as_ref` 関数内で使用します。
///
/// # 引数
///
/// * `s` - `AsRef<str>` トレイトを実装している任意の型。
///
/// # 利用例
///
/// ```
/// let s = String::from("Hello, world!");
/// print_as_ref(s);
/// ```
pub fn print_as_ref<T: AsRef<str>>(s: T) {
    let s = s.as_ref();
    println!("{}", s);
}