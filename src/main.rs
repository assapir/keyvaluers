mod key_value;

use key_value::KeyValue;

fn main() {
    let kv = KeyValue::new("assaf", "sapir");
    println!("{}", kv);
}
