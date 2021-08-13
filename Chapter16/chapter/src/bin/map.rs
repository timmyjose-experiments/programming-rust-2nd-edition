use std::collections::{BTreeMap, HashMap};

fn main() {
    let map1: HashMap<String, i32> = HashMap::new();
    let mut map2 = [
        ("hello", 1),
        ("world", 2),
        ("we", 3),
        ("meet", 4),
        ("again", 5),
    ]
    .iter()
    .map(|(k, v)| (k.to_string(), v))
    .collect::<BTreeMap<_, _>>();

    for (k, v) in map2.iter() {
        println!("{} => {}", k, v);
    }
    println!();

    assert_eq!(map1.len(), 0);
    assert_eq!(map2.len(), 5);

    assert!(map2.contains_key("hello"));
    assert!(!map2.contains_key("hallo"));

    map2.insert("hola".to_string(), &100);
    assert!(map2.len() == 6);

    let v1 = map2.remove("hello");
    assert_eq!(v1, Some(&1));

    if let Some((k2, v2)) = map2.remove_entry("world") {
        assert_eq!(k2, "world");
        assert_eq!(v2, &2);
    }

    let v3 = map2.entry("we".to_string()).or_insert(&42);
    assert_eq!(**v3, 3);

    let _ = map2.entry("hallo".to_string()).or_insert(&42);

    for (k, v) in &map2 {
        println!("{} => {}", k, v);
    }
}