use std::collections::HashMap;

fn main() {
    let mut h = HashMap::new();
    for i in 0..10 {
        h.insert(i, i);
    }
    // for key in h.keys() {
    //     if key % 3 == 0 {
    //         h.remove(key);
    //     }
    // }
    let h: HashMap<_, _> = h.into_iter().filter(|(key, _)| key % 3 != 0).collect();
    println!("filtered={:?}", h);
}
