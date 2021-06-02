use std::collections::HashMap;
use crate::key_value::KeyValue;

#[derive(Debug)]
struct Holder<T> {
    map: HashMap<String, KeyValue<T>>
}
