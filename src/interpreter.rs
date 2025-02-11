use std::collections::HashMap;

use crate::{logger, parser, SorbetError};

pub fn do_action(action_map: HashMap<String, String>) {
    let action = action_map.keys().next().unwrap();
    let action_input = action_map.values().next().unwrap();

    match action.as_str() {
        "print" => println!("{}", action_input),
        _ => {
            logger::print_error(
                SorbetError::Syntax,
                format!("Expected keyword, found: {}", action)
            );
            std::process::exit(1);
        }
    }
}

pub fn interpret(line: String, execute: bool) -> HashMap<String, String> {
    let line_map: HashMap<String, String> = parser::parse(line);
    let current_action = line_map.keys().next().unwrap();

    match current_action.as_str() {
        "print" => {
            if execute {
                do_action(line_map.clone());
            }
            line_map
        },
        _ => {
            logger::print_error(
                SorbetError::Syntax,
                format!("Expected keyword, found: {}", current_action)
            );
            std::process::exit(1);
        }
    }
}