
/// # 概要
/// このプログラムは、`Selector`構造体を使用して、要素の選択と変更を行う例です。
///
/// # 詳細
/// `Selector`構造体は、`elements`ベクタと`current`インデックスを持ち、現在の要素を指し示します。
/// `Deref`と`DerefMut`トレイトを実装することで、`*selector`のようなシンタックスで要素にアクセスおよび変更が可能です。
///
/// # 使用例
/// ```rust
/// let mut selector = Selector {
///     elements: vec!["one", "two", "three"],
///     current: 2,
/// };
///
/// println!("Current element: {}", *selector);
/// *selector = "four";
/// println!("Current element: {}", *selector);
/// ```
///
/// # テスト
/// このプログラムには、`Selector`の`Deref`および`DerefMut`の動作を確認するためのテストも含まれています。
///
/// ```rust
/// #[test]
/// fn test_selector_deref() {
///     let mut selector = Selector {
///         elements: vec!["one", "two", "three"],
///         current: 2,
///     };
///
///     let current = *selector;
///     assert_eq!(current, "three");
/// }
///
/// #[test]
/// fn test_selector_deref_mut() {
///     let mut selector = Selector {
///         elements: vec![1, 2, 3],
///         current: 1,
///     };
///
///     let current = selector.deref_mut();
///     *current = 5;
///     assert_eq!(*current, 5);
/// }
/// ```
use chapter13::deref::Selector;

fn main() {
    let mut selector = Selector {
        elements: vec!["one", "two", "three"],
        current: 2,
    };

    println!("Current element: {}", *selector);
    *selector = "four";
    println!("Current element: {}", *selector);
}

#[cfg(test)]
mod tests {
    use std::ops::DerefMut;

    use super::*;

    #[test]
    fn test_selector_deref() {
        let mut selector = Selector {
            elements: vec!["one", "two", "three"],
            current: 2,
        };

        let current = *selector;
        assert_eq!(current, "three");
    }

    #[test]
    fn test_selector_deref_mut() {
        let mut selector = Selector {
            elements: vec![1, 2, 3],
            current: 1,
        };

        let current = selector.deref_mut();
        *current = 5;
        assert_eq!(*current, 5);
    }
}