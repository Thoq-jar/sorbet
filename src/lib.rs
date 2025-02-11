mod logger;
mod parser;
mod interpreter;

pub use logger::SorbetError;
pub use parser::*;
pub use interpreter::interpret;

#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use super::*;

    #[test]
    fn test_parser() {
        let data = String::from("test => hi");
        let result = parse(data);
        let mut expected: HashMap<String, String> = HashMap::new();
        expected.insert("test".to_string(), "hi".to_string());

        assert_eq!(result, expected);
    }

        #[test]
    fn test_interpreter() {
        let data = String::from("print => Sorbet");
        let result = interpret(data, false);
        let mut expected: HashMap<String, String> = HashMap::new();
        expected.insert("print".to_string(), "Sorbet".to_string());
    
        assert_eq!(result, expected);
    }
}
