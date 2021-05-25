use std::fmt;

#[derive(Debug, PartialEq, Eq)]
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

impl<T: fmt::Display> fmt::Display for KeyValue<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} = {}", self.key, self.value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from_str() {
        let kv = KeyValue::from_str("key", "value");
        assert_eq!(kv.key, String::from("key"));
        assert_eq!(kv.value, String::from("value"));
        assert_ne!(kv.key, String::from("value"));
    }

    #[test]
    fn from_string() {
        let kv = KeyValue::from_string(String::from("key"), 2);
        assert_eq!(kv.key, String::from("key"));
        assert_eq!(kv.value, 2);
    }

    #[test]
    fn partial_eq_eq() {
        let kv1 = KeyValue::from_str("key", 1);
        let kv2 = KeyValue::from_string(String::from("key"),1);
        assert_eq!(kv1, kv2)
    }
    #[test]
    fn partial_eq_ne() {
        let kv1 = KeyValue::from_str("key", 2);
        let kv2 = KeyValue::from_string(String::from("key"),1);
        assert_ne!(kv1, kv2)
    }
}
