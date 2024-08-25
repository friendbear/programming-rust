/// First in First out data structure
#[derive(Default, Debug)]
pub struct Queue<T> {
    older: Vec<T>,
    younger: Vec<T>,
}

impl<T> Queue<T> {
    /// 所有権の取得 (selfを消費する)
    pub fn split(self) -> (Vec<T>, Vec<T>) {
        (self.older, self.younger)
    }
    /// 参照の取得 (selfを消費しない)
    pub fn len(&self) -> usize {
        self.older.len() + self.younger.len()
    }
    pub fn is_empty(&self) -> bool {
        self.older.is_empty() && self.younger.is_empty()
    }
    /// &mut Queue<T> に対するメソッド
    pub fn push(&mut self, v: T) {
        self.younger.push(v);
    }
    ///
    /// # Returns
    /// - `Some(T)` if the queue is not empty
    /// - `None` if the queue is empty
    ///
    /// ```
    /// use implimentation::Queue;
    /// let mut q = Queue::<u32>::default();
    /// assert!(q.len() == 0);
    /// assert!(q.is_empty());
    /// q.push(42);
    /// assert!(q.len() == 1);
    /// assert!(!q.is_empty());
    ///
    /// assert_eq!(q.pop(), Some(42));
    /// assert!(q.len() == 0);
    /// assert!(q.is_empty());
    /// ```
    pub fn pop(&mut self) -> Option<T> {
        if self.older.is_empty() {
            if self.younger.is_empty() {
                return None;
            } else {
                std::mem::swap(&mut self.older, &mut self.younger);
                self.older.reverse();
            }
        }
        self.older.pop()
    }
}

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    #[test]
    fn it_works_queue() {
        let mut q = Queue::<u32>::default();
        assert!(q.len() == 0);
        assert!(q.is_empty());
        q.push(1);
        q.push(2);
        assert_eq!(q.pop(), Some(1));
        assert_eq!(q.pop(), Some(2));
        assert_eq!(q.pop(), None);
    }
}
