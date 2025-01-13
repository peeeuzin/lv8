use std::fmt::Debug;

pub struct Error {
    pub message: String,
    pub kind: ErrorKind,
}

impl Error {
    pub fn syntax(message: &str, syntax: SyntaxError) -> Self {
        Self {
            message: message.to_string(),
            kind: ErrorKind::Syntax(syntax),
        }
    }

    pub fn reference(message: &str) -> Self {
        Self {
            message: message.to_string(),
            kind: ErrorKind::Reference,
        }
    }

    pub fn r#type(message: &str) -> Self {
        Self {
            message: message.to_string(),
            kind: ErrorKind::Type,
        }
    }
}

impl Debug for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.kind {
            ErrorKind::Syntax(error) => write!(f, "SyntaxError: {}\n{:?}", self.message, error),
            ErrorKind::Reference => write!(f, "ReferenceError: {}", self.message),
            ErrorKind::Type => write!(f, "TypeError: {}", self.message),
        }
    }
}

pub enum ErrorKind {
    Syntax(SyntaxError),
    Reference,
    Type,
}

pub struct SyntaxError {
    pub line: String,
    pub line_pos: usize,
    pub column_pos: usize,
}

impl Debug for SyntaxError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "  --> at line {}:{}\n  |\n{} | {}\n  |{}^\n",
            self.line_pos,
            self.column_pos,
            self.line_pos,
            self.line,
            " ".repeat(self.column_pos),
        )
    }
}

impl SyntaxError {
    pub fn new(line: &str, line_pos: usize, column_pos: usize) -> Self {
        Self {
            line: line.to_string(),
            line_pos,
            column_pos,
        }
    }
}

pub type Result<T> = std::result::Result<T, Error>;
