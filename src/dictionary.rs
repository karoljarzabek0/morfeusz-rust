use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};


pub fn init_dictionary(path: &str) -> HashMap<String, String> {
    let mut ht = HashMap::new();

    let file = File::open(path).expect("Failed to open .dic file"); 
    let lines = io::BufReader::new(file).lines();

    for line in lines {
        let line = line.unwrap();
        let parts: Vec<&str> = line.split("/").collect();
        let word: String = parts[0].to_string();
        let rules: String = parts.get(1).cloned().unwrap_or_default().to_string();
        ht.insert(word, rules);
        }

    return ht;
}