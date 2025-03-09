use std::collections::HashMap;
use std::fs;
use sorbet_kvp::sorbet;
use std::path::Path;
use configparser::ini::Ini;

fn get_output_file(input_file: &str, extension: &str, output: Option<&str>) -> String {
    match output {
        Some(out) => out.to_string(),
        None => format!("{}{}", input_file.trim_end_matches(Path::new(input_file).extension().unwrap_or_default().to_str().unwrap_or("")), extension)
    }
}

pub fn convert_to_json(sorbet_file: &str, output: Option<&str>) {
    let new_name = get_output_file(sorbet_file, "json", output);
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

pub fn convert_to_kvp(sorbet_file: &str, extension: &str, output: Option<&str>) {
    let new_name = get_output_file(sorbet_file, extension, output);
    let contents: String = fs::read_to_string(sorbet_file).unwrap();
    let content_map: HashMap<String, String> = sorbet::parse(contents);
    let properties = content_map.iter()
        .map(|(k, v)| format!("{}=\"{}\"", k, v.replace('\n', "\\n")))
        .collect::<Vec<String>>().join("\n");
    fs::write(new_name, properties).unwrap();
}

pub fn convert_to_xml(sorbet_file: &str, output: Option<&str>) {
    let new_name = get_output_file(sorbet_file, "xml", output);
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

pub fn convert_to_sectioned_kvp(sorbet_file: &str, extension: &str, output: Option<&str>) {
    let new_name = get_output_file(sorbet_file, extension, output);
    let contents: String = fs::read_to_string(sorbet_file).unwrap();
    let content_map: HashMap<String, String> = sorbet::parse(contents);
    let properties = format!(
        "[sorbet]\n{}",
        content_map.iter()
        .map(|(k, v)| format!("{} = \"{}\"", k, v.replace('\n', "\\n")))
        .collect::<Vec<String>>().join("\n"));
    fs::write(new_name, properties).unwrap();
}

pub fn convert_to_sorbet(file: &str, output: Option<&str>) {
    let path = Path::new(file);
    let extension = path.extension()
        .and_then(|ext| ext.to_str())
        .unwrap_or("");
    
    let contents = fs::read_to_string(file).unwrap_or_else(|_| panic!("Could not read file {}", file));
    let content_map: HashMap<String, String> = match extension {
        "json" => {
            serde_json::from_str::<HashMap<String, String>>(&contents)
                .unwrap_or_else(|_| panic!("Invalid JSON format"))
        },
        "xml" => {
            let doc = roxmltree::Document::parse(&contents).unwrap();
            let mut map = HashMap::new();
            for node in doc.root_element().children().filter(|n| n.is_element()) {
                map.insert(node.tag_name().name().to_string(), node.text().unwrap_or("").to_string());
            }
            map
        },
        "properties" | "env" => {
            contents.lines()
                .filter_map(|line| {
                    let parts: Vec<&str> = line.splitn(2, '=').collect();
                    if parts.len() == 2 {
                        Some((
                            parts[0].trim().to_string(),
                            parts[1].trim().trim_matches('"').to_string()
                        ))
                    } else {
                        None
                    }
                })
                .collect()
        },
        "toml" | "ini" => {
            let mut config = Ini::new();
            config.read(contents).unwrap();
            let map_ref = config.get_map_ref();
            
            let section_data = map_ref.get("sorbet")
                .or_else(|| map_ref.get("DEFAULT"))
                .unwrap_or_else(|| panic!("No valid section found in INI file"));

            section_data.iter()
                .filter_map(|(k, v)| {
                    v.as_ref().map(|val| (k.clone(), val.clone()))
                })
                .collect()
        },
        _ => panic!("Unsupported input format!")
    };

    let sorbet_content = content_map.iter()
        .map(|(k, v)| format!("{}={}", k, v))
        .collect::<Vec<String>>()
        .join("\n");

    let output_file = get_output_file(file, "srb", output);
    fs::write(output_file, sorbet_content).unwrap();
}
