mod utility;
mod parser;
pub mod sorbet;

#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use crate::utility::check_file_extension;
    use super::sorbet::*;

    #[test]
    fn test_parser() {
        let data: String = String::from("test => hi");
        let result: HashMap<String, String> = parse(data);
        let mut expected: HashMap<String, String> = HashMap::new();
        expected.insert("test".to_string(), "hi".to_string());

        assert_eq!(result, expected);
    }

    #[test]
    fn test_parser_multiline() {
        let data: String = String::from(
            "test => line 1\n\
             > line 2\n\
             > line 3"
        );
        let result: HashMap<String, String> = parse(data);
        let mut expected: HashMap<String, String> = HashMap::new();
        expected.insert(
            "test".to_string(),
            "line 1\nline 2\nline 3".to_string()
        );
    
        assert_eq!(result, expected);
    }

    #[test]
    fn test_check_file_extension() {
        let file_extension_srb: String = String::from("test.srb");
        let file_extension_sorbet: String = String::from("test.sorbet");
        let file_extension_txt: String = String::from("test.txt");

        let result_srb: bool = check_file_extension(file_extension_srb);
        let result_sorbet: bool = check_file_extension(file_extension_sorbet);
        let result_txt: bool = check_file_extension(file_extension_txt);

        assert_eq!(result_srb, true);
        assert_eq!(result_sorbet, true);
        assert_eq!(result_txt, false);
    }
}
