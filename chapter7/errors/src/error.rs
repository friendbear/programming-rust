
#[derive(Debug, Clone)]
pub struct JsonError {
    pub message: String,
    pub line: usize,
    pub column: usize,
}

impl fmt::Display for JsonError {
    fn fmt(&self, f: &mut fmt::Formatter) {
        write!(f, "{}, {}, {}", self.message, self. line, self.column)
    }
}

impl std::error::Error for JsonError {
    fn description(&self) -> &str {
        &self.message
    }
}