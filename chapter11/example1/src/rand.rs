use rand::random;
use rand::Rng;

#[cfg(test)]
mod tests {
    use rand::Rng;

    use super::*;

    #[test]
    fn test_random() {
       // u32の範囲を確認するテスト
       let value = rand::random::<u32>();
       assert!(value <= u32::MAX); // u32::MAX2^32 - 1
   
       let b = rand::random::<bool>();
       // ここで具体的にtrueかfalseかを確認するか、削除
   
       let mut rng = rand::thread_rng();
       let value = rng.gen_range(0..10);
       assert!(value < 10);
   
       // 例えば、特定のが生成されることを確認するテスト
       let mut rng = rand::thread_rng();
       let value = rng.gen_range(0..10);
       assert!(value >= 0 && value < 10);
   }
}