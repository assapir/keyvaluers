use keyvaluers::key_value::KeyValue;

fn main() {
    let kv = KeyValue::from_str("assaf", 2);
    let as_string = kv.to_string();
    print!("{}", as_string);
}
