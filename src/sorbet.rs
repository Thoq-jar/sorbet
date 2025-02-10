use std::collections::HashMap;
use crate::logger;
use crate::logger::SorbetError;

pub fn parse(contents: String) -> HashMap<String, String> {
    let mut map: HashMap<String, String> = HashMap::new();

    for line in contents.lines() {
        let parts: Vec<&str> = line.split("=>").collect();

        if parts.len() == 2 {
            let task = parts[0].trim().to_string();
            let command = parts[1].trim().to_string();

            map.insert(task, command);
        } else {
            logger::print_error(
                SorbetError::Syntax,
                format!("Syntax error! Expected key => value at: {}", line)
            );
        }
    }

    map
}
