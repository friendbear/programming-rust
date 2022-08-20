
pub struct Queue<T> {
    pub older: Vec<T>,
    pub younger: Vec<T>
}

impl<T> Queue<T> {
    pub fn new() -> Self {
        Queue { older: Vec::new(), younger: Vec::new()}
    }
    pub fn push(&mut self, v: T) {
        self.younger.push(v)
    }

    pub fn is_emply(&self) -> bool {
        self.older.is_empty() && self.younger.is_empty()
    }
}