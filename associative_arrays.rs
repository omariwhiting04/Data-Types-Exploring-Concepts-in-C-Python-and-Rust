use std::collections::HashMap;
fn main() {
    let mut scores = HashMap::new();
    scores.insert("Alice", 90);
    scores.insert("Bob", 85);
    for (k,v) in &scores { println!("{} {}", k, v); }
}
