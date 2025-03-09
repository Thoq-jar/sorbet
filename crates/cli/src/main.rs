mod utility;

use crate::utility::{
    convert_to_json,
    convert_to_kvp,
    convert_to_sectioned_kvp,
    convert_to_xml,
    convert_to_sorbet
};

use sorbet_kvp::sorbet;

fn main() {
    let args: Vec<String> = std::env::args().collect::<Vec<String>>();

    if args.len() < 2 {
        println!("Usage: sorbet [convert/to-sorbet] [file] [format]");
        std::process::exit(1);
    }

    if args.len() < 3 {
        println!("Please provide a file to convert!");
        println!("Usage: sorbet [convert/to-sorbet] [file] [format]");
        std::process::exit(1);
    }

    let input_file: String = args[2].clone();

    match args[1].as_str() {
        "convert" => {
            if args.len() < 4 {
                println!("Please provide a format to convert to!");
                println!("Usage: sorbet convert [file.srb] [json/xml/properties/env/toml/ini] [output_file?]");
                std::process::exit(1);
            }

            if !sorbet::check_file_extension(input_file.clone()) {
                println!("Invalid sorbet file extension! (Use .srb or .sorbet)");
                std::process::exit(1);
            }

            let output = args.get(4);
            match args[3].as_str() {
                "json" => convert_to_json(&*input_file, output.map(|s| s.as_str())),
                "xml" => convert_to_xml(&*input_file, output.map(|s| s.as_str())),
                "properties" => convert_to_kvp(&*input_file, "properties", output.map(|s| s.as_str())),
                "env" => convert_to_kvp(&*input_file, "env", output.map(|s| s.as_str())),
                "toml" => convert_to_sectioned_kvp(&*input_file, "toml", output.map(|s| s.as_str())),
                "ini" => convert_to_sectioned_kvp(&*input_file, "ini", output.map(|s| s.as_str())),
                _ => println!("Invalid format! (Use one of json, xml, properties, ini, toml, or env)"),
            }
        }
        "to-sorbet" => {
            let output = args.get(3);
            convert_to_sorbet(&*input_file, output.map(|s| s.as_str()))
        },
        _ => println!("Invalid option! Use 'convert' or 'to-sorbet'"),
    }
}
