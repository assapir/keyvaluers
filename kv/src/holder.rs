use crate::key_value::KeyValue;
use std::collections::HashMap;

#[derive(Debug)]
struct Holder<T> {
    map: HashMap<String, KeyValue<T>>,
}
