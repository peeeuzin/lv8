pub type Result<T> = std::result::Result<T, GrammarError>;

#[derive(Debug)]
pub struct GrammarError(pub String);

impl GrammarError {
    pub fn with_message(msg: &str) -> Self {
        Self(msg.to_string())
    }
}

impl std::fmt::Display for GrammarError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
