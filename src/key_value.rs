use std::fmt;

#[derive(Debug)]
pub struct KeyValue<T> {
    key: String,
    value: T,
}

impl<T> KeyValue<T> {
    pub fn from_str(key: &str, value: T) -> KeyValue<T> {
        KeyValue {
            key: String::from(key),
            value,
        }
    }

    pub fn from_string(key: String, value: T) -> KeyValue<T> {
        KeyValue { key, value }
    }
}

impl<T> fmt::Display for KeyValue<T>
where
    T: fmt::Display,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} = {}", self.key, self.value)
    }
}
