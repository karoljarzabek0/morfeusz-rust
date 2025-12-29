use std::env;

mod aff;
mod dictionary;

fn main() {
    let args: Vec<String> = env::args().collect();
    let query = args.get(1).expect("Please provide a query word as a command line argument.");  
    
    let dict = dictionary::init_dictionary("pl_PL.dic");

    let Ok(rules1) = aff::AffRules::from_path("pl_PL.aff") else {
        eprintln!("Error: Could not load the .aff file.");
        return;
    };

    let all_perumtations = rules1.apply_rules(query, &dict.get(query).unwrap_or(&"".to_string()));

    all_perumtations.iter().for_each(|perm| println!("{}", perm));            
}