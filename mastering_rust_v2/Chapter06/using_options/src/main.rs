use std::collections::HashMap;

fn main() {
    let mut map = HashMap::new();
    map.insert("one", 1);
    map.insert("two", 2);
    let value = map.get("one").unwrap();
    let incremented_value = value + 1;
}
