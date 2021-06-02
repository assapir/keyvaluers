use keyvaluers::key_value::KeyValue;

fn main() {
    let kv = KeyValue::from_str("assaf", 2);
    print!("{}", kv);
}
