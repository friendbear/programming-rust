pub struct Application{ 
    pub name: String,
    pub nickname: Vec<String>,
}

impl std::ops::Drop for Application {
    fn drop(&mut self) {
        print!("Dropping {}", self.name);
        if !self.nickname.is_empty() {
            print!("(AKA{})", self.nickname.join(", "))
        }
        println!("");
    }
}

pub fn add(left: usize, right: usize) -> usize {
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
}
