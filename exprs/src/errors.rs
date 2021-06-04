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

#[derive(Debug)]
pub struct ReSendError {
    pub message: &'static str,
}

impl ReSendError {
    pub(crate) fn new() -> Self {
        Self {
            message: "It's impossible to send again response that was already sent.",
        }
    }
}
