
use std::fs::File;
use std::io::{self, BufRead};

#[derive(Debug)]
pub struct AffNode {
    remove: String,
    add: String,
    condition: String,
}

#[derive(Debug)]
pub struct AffGroup {
    rule_type: String,
    flag: String,
    cross_product: bool, // Changed to bool for better logic
    children: Vec<AffNode>, // Removed n_children (redundant with children.len())
}

#[derive(Debug)]
pub struct AffRules {
    pub rules: Vec<AffGroup>,
}

impl AffRules {

    pub fn from_path(path: &str) -> io::Result<AffRules> {
        let mut aff_rules: Vec<AffGroup> = vec![];
        
        let file = File::open(path)?; 
        let lines = io::BufReader::new(file).lines();

        for line in lines {
            let line = line?;
            let parts: Vec<&str> = line.split_whitespace().collect();

            if parts.is_empty() { continue; }

            // Headers have 4 parts: TYPE FLAG CROSS_PRODUCT COUNT
            if parts.len() == 4 && (parts[2] == "Y" || parts[2] == "N") {
                let group = AffGroup {
                    rule_type: parts[0].to_string(),
                    flag: parts[1].to_string(),
                    cross_product: parts[2] == "Y",
                    children: Vec::with_capacity(parts[3].parse().unwrap_or(0)),
                };
                aff_rules.push(group);
            } 
            // Rules have 5 parts: TYPE FLAG REMOVE ADD CONDITION
            else if parts.len() >= 5 {
                let rule_type = parts[0];
                let flag = parts[1];

                let node = AffNode {
                    remove: parts[2].to_string(),
                    add: parts[3].to_string(),
                    condition: parts[4].to_string(),
                };

                if let Some(last_group) = aff_rules.last_mut() {
                    if last_group.rule_type == rule_type && last_group.flag == flag {
                        last_group.children.push(node);
                    } else {
                        eprintln!("Orphan rule found: {:?}", parts);
                    }
                }
            }
        }

        Ok(AffRules { rules: aff_rules })
    }

    fn apply_rule(&self, word: &str, flag: &str) -> Vec<String> {
        let mut possible_permutations: Vec<String> = vec![];

        if let Some(group) = self.rules.iter().find(|g| g.flag == flag) {
            
            // Iterate over every rule in that group
            for rule in &group.children {
                
                // Check Condition -> Remove -> Add
                // TO DO: impelement regex rule mathing - currently there is no check
                if rule.condition == "." || word.ends_with(&rule.condition) || true {
                    
                    // "0" means "remove nothing"
                    let to_remove = if rule.remove == "0" { "" } else { &rule.remove };
                    
                    if word.ends_with(to_remove) || true {
                        // Strip the end
                        let base = &word[..word.len() - to_remove.len()];
                        
                        // Add the new ending
                        let new_word = format!("{}{}", base, rule.add);
                        
                        possible_permutations.push(new_word);
                    }
                }
            }
        }

        possible_permutations
    }

    pub fn apply_rules(&self, word: &str, rules: &str) -> Vec<String> {
        let mut all_permutations: Vec<String> = vec![];

        for rule in rules.chars() {
            let mut possible_permutations: Vec<String> = self.apply_rule(word, &rule.to_string());
            // DEBUG possible_permutations.iter().for_each(|perm| println!("{}, rule: {}", perm, rule));  
            all_permutations.append(&mut possible_permutations);
        }

        all_permutations

    }
}