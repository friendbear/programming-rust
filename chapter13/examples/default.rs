
/// `patition_of_powers_of_two` 関数は、与えられた整数のスライスを2つの`HashSet`に分割します。
/// 1つは2のべき乗である整数の集合、もう1つはそれ以外の整数の集合です。
///
/// # 引数
///
/// * `squares` - 整数のスライス。このスライス内の整数が2つの集合に分割されます。
///
/// # 戻り値
///
/// `(HashSet<i32>, HashSet<i32>)` - 最初の`HashSet`は2のべき乗である整数を含み、
/// 2番目の`HashSet`はそれ以外の整数を含みます。
///
/// # 例
///
/// ```
/// let squares = [4, 9, 16, 25, 36, 49, 64];
/// let (powers_of_two, others) = patition_of_powers_of_two(&squares);
/// assert_eq!(powers_of_two.len(), 3);
/// assert_eq!(others.len(), 4);
/// ```
///
/// # 注意
///
/// この関数は`Default`トレイトを必要としません。`partition`メソッドはイテレータを2つのコレクションに分割するために使用され、
/// それぞれのコレクションは`Default`トレイトを実装している必要がありますが、`HashSet`は`Default`を実装しているため、
/// 特に問題はありません。
use std::collections::HashSet;
fn main() {

    let squares = [4, 9, 16, 25, 36, 49, 64];

    let (powers_of_two, others): (HashSet<i32>, HashSet<i32>) = patition_of_powers_of_two(&squares);
    println!("Powers of two: {:?}", powers_of_two);
    println!("Others: {:?}", others);

}

fn patition_of_powers_of_two(squares: &[i32]) -> (HashSet<i32>, HashSet<i32>) {
    squares.iter().partition(|&n| n & (n -1) == 0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_partition_of_powers_of_two() {
        let squares = [4, 9, 16, 25, 36, 49, 64];
        let (powers_of_two, others) = patition_of_powers_of_two(&squares);
        assert_eq!(powers_of_two.len(), 3);
        assert_eq!(others.len(), 4);
    }
    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn test_partition_of_powers_of_two() {
            let squares = [4, 9, 16, 25, 36, 49, 64];
            let (powers_of_two, others) = patition_of_powers_of_two(&squares);
            assert_eq!(powers_of_two.len(), 3);
            assert_eq!(others.len(), 4);
        }

        #[test]
        fn test_partition_with_no_powers_of_two() {
            let squares = [3, 5, 7, 9, 11];
            let (powers_of_two, others) = patition_of_powers_of_two(&squares);
            assert_eq!(powers_of_two.len(), 0);
            assert_eq!(others.len(), 5);
        }

        #[test]
        fn test_partition_with_all_powers_of_two() {
            let squares = [1, 2, 4, 8, 16, 32, 64];
            let (powers_of_two, others) = patition_of_powers_of_two(&squares);
            assert_eq!(powers_of_two.len(), 7);
            assert_eq!(others.len(), 0);
        }

        #[test]
        fn test_partition_with_empty_input() {
            let squares: [i32; 0] = [];
            let (powers_of_two, others) = patition_of_powers_of_two(&squares);
            assert_eq!(powers_of_two.len(), 0);
            assert_eq!(others.len(), 0);
        }

        #[test]
        fn test_partition_with_mixed_numbers() {
            let squares = [1, 3, 4, 6, 8, 10, 16, 18, 32, 33];
            let (powers_of_two, others) = patition_of_powers_of_two(&squares);
            assert_eq!(powers_of_two.len(), 5);
            assert_eq!(others.len(), 5);
        }
    }
}