mod logger;
mod sorbet;

pub use logger::SorbetError;
pub use sorbet::*;

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
}
