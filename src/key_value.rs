use std::fmt;

#[derive(Debug)]
pub struct KeyValue {
    key: String,
    value: String,
}

impl KeyValue {
    pub fn new(key: &str, value: &str) -> KeyValue {
        KeyValue {
            key: String::from(key),
            value: String::from(value)
        }
    }
}

impl fmt::Display for KeyValue {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} = {}", self.key, self.value)
    }
}
