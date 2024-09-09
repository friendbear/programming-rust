use rand::random;
use rand::Rng;

#[cfg(test)]
mod tests {
    use rand::Rng;

    use super::*;

    #[test]
    fn test_random() {
        let value = random::<u32>();
        assert!(value < 2u32.pow(32));

        let b = random::<bool>();
        assert!(b == true || b == false);

        let mut rng = rand::thread_rng();
        let value = rng.gen_range(0..10);
        assert!(value < 10);
    }
}