use crate::utility::{
    convert_to_json,
    convert_to_kvp,
    convert_to_sectioned_kvp,
    convert_to_xml
};

mod utility;

fn main() {
    let args: Vec<String> = std::env::args().collect::<Vec<String>>();

    if args.len() < 2 {
        println!("Usage: sorbet [convert] [file.srb] [json/xml/properties]");
        std::process::exit(1);
    }

    if args.len() < 3 {
        println!("Please provide a sorbet file to convert!");
        println!("Usage: sorbet [convert] [file.srb] [json/xml/properties]");
        std::process::exit(1);
    }

    if args.len() < 4 {
        println!("Please provide a format to convert to!");
        println!("Usage: sorbet [convert] [file.srb] [json/xml/properties]");
        std::process::exit(1);
    }

    let sorbet_file: String = args[2].clone();
    if !sorbet::check_file_extension(sorbet_file.clone()) {
        println!("Invalid sorbet file extension! (Use .srb or .sorbet)");
        std::process::exit(1);
    }

    match args[1].as_str() {
        "convert" => {
            match args[3].as_str() {
                "json" => convert_to_json(&*sorbet_file),
                "xml" => convert_to_xml(&*sorbet_file),
                "properties" => convert_to_kvp(&*sorbet_file, "properties"),
                "env" => convert_to_kvp(&*sorbet_file, "env"),
                "toml" => convert_to_sectioned_kvp(&*sorbet_file, "toml"),
                "ini" => convert_to_sectioned_kvp(&*sorbet_file, "ini"),
                _ => println!("Invalid format! (Use one of json, xml, properties)"),
            }
        }
        _ => println!("Invalid option! (Run with no args for help)"),
    }
}
