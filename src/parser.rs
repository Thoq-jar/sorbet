use std::collections::HashMap;
use crate::logger;
use crate::logger::SorbetError;


pub fn parse(contents: String) -> HashMap<String, String> {
    let mut map: HashMap<String, String> = HashMap::new();
    let mut current_key: Option<String> = None;
    let mut current_value = String::new();

    for line in contents.lines() {
        if line.contains("=>") {
            if let Some(ref key) = current_key {
                map.insert(key.clone(), current_value.trim().to_string());
                current_value.clear();
            }
            
            let parts: Vec<&str> = line.split("=>").collect();
            if parts.len() == 2 {
                current_key = Some(parts[0].trim().to_string());
                current_value = parts[1].trim().to_string();
            } else {
                logger::print_error(
                    SorbetError::Syntax,
                    format!("Syntax error! Expected [key] => [value] at: {}", line)
                );
            }
        } else if line.trim().starts_with('>') {
            if current_key.is_some() {
                current_value.push('\n');
                current_value.push_str(line.trim_start().trim_start_matches('>').trim());
            } else {
                logger::print_error(
                    SorbetError::Syntax, 
                    format!("Continuation line without a key at: {}", line)
                );
            }
        }
    }

    if let Some(key) = current_key {
        map.insert(key, current_value.trim().to_string());
    }

    map
}
