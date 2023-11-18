use std::{collections::HashMap, io};

fn main() {
    println!("Enter key-value pairs separated by a colon");
    println!("Enter keys for the first map. When you are done, enter \"next\"");
    let map1 = load_map();
    println!();
    println!("Enter key-value pairs for the second map");
    let map2 = load_map();
    println!();
    println!("Key: Map 1, Map 2");
    for (key, value) in map1 {
        if Some(&value) != map2.get(&key) {
            println!("{}: {}, {}", key, value, map2.get(&key).unwrap_or(&"".to_owned()))
        }
    }
}

fn load_map() -> HashMap<String, String> {
    let mut map: HashMap<String, String> = HashMap::new();
    loop {
        let mut kv_pair = String::new();
        io::stdin().read_line(&mut kv_pair).unwrap();
        kv_pair = kv_pair.trim().to_owned();
        if kv_pair.to_lowercase() == "next" {
            break;
        }
        if let Some((key, value)) = kv_pair.trim().split_once(':') {
            map.insert(key.to_owned(), value.to_owned());
        }
    }
    map
}
