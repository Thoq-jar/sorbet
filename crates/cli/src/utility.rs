use std::collections::HashMap;
use std::fs;

pub fn convert_to_json(sorbet_file: &str) {
    let new_name: String = format!("{}.json", sorbet_file.trim_end_matches(".srb").trim_end_matches(".sorbet"));
    let contents: String = fs::read_to_string(sorbet_file).unwrap();
    let content_map: HashMap<String, String> = sorbet::parse(contents);
    let json = format!(
        "{{{}}}",
        content_map.iter()
            .map(|(k, v)| format!("\"{}\": \"{}\"", k, v.replace('\n', "\\n")))
            .collect::<Vec<String>>().join(",")
    );
    fs::write(new_name, json).unwrap();
}

pub fn convert_to_kvp(sorbet_file: &str, extension: &str) {
    let new_name: String = format!("{}.{}", sorbet_file.trim_end_matches(".srb").trim_end_matches(".sorbet"), extension);
    let contents: String = fs::read_to_string(sorbet_file).unwrap();
    let content_map: HashMap<String, String> = sorbet::parse(contents);
    let properties = content_map.iter()
        .map(|(k, v)| format!("{}=\"{}\"", k, v.replace('\n', "\\n")))
        .collect::<Vec<String>>().join("\n");
    fs::write(new_name, properties).unwrap();
}

pub fn convert_to_xml(sorbet_file: &str) {
    let new_name: String = format!("{}.xml", sorbet_file.trim_end_matches(".srb").trim_end_matches(".sorbet"));
    let contents: String = fs::read_to_string(sorbet_file).unwrap();
    let content_map: HashMap<String, String> = sorbet::parse(contents);

    let xml = format!(
        "<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n<sorbet>\n{}\n</sorbet>",
        content_map.iter()
            .map(|(k, v)| format!("  <{}>{}</{}>", k, v.replace('\n', "&#10;"), k))
            .collect::<Vec<String>>()
            .join("\n")
    );

    fs::write(new_name, xml).unwrap();
}

pub fn convert_to_sectioned_kvp(sorbet_file: &str, extension: &str) {
    let new_name: String = format!("{}.{}", sorbet_file.trim_end_matches(".srb").trim_end_matches(".sorbet"), extension);
    let contents: String = fs::read_to_string(sorbet_file).unwrap();
    let content_map: HashMap<String, String> = sorbet::parse(contents);
    let properties = format!(
        "[sorbet]\n{}",
        content_map.iter()
        .map(|(k, v)| format!("{} = \"{}\"", k, v.replace('\n', "\\n")))
        .collect::<Vec<String>>().join("\n"));
    fs::write(new_name, properties).unwrap();
}
