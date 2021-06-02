#[derive(Debug)]
pub struct ParseError {
    pub message: String,
}

impl ParseError {
    pub(crate) fn new(message: &str) -> Self {
        Self {
            message: String::from(message),
        }
    }
}
