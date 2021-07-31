struct StringTable {
    elements: Vec<String>,
}

impl StringTable {
    fn find_by_prefix(&self, prefix: &str) -> Option<&String> {
        self.elements
            .iter()
            .filter(|&s| s.starts_with(prefix))
            .nth(0)
    }
}

fn main() {
    let table = StringTable {
        elements: vec![
            "hello".to_string(),
            "hella".to_string(),
            "health".to_string(),
            "welcome".to_string(),
            "again".to_string(),
        ],
    };

    println!("{:?}", table.find_by_prefix("he"));
    println!("{:?}", table.find_by_prefix("be"));
}
