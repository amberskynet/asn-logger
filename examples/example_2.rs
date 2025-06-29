extern crate asn_logger;

use asn_logger::AsnLogConfig;
use std::fs;

const PATH: &str = "./examples/log_config.json";

fn main() {
    let json_str = r#"
        {
            "global_level": "Info",
            "module_levels": {
                "module1": "Debug",
                "module2": "Error"
            }
        }
    "#;

    let config_result = AsnLogConfig::from_json(json_str);

    match config_result {
        Ok(config) => {
            println!("Parsed config: {config:?}");
        }
        Err(e) => eprintln!("Error parsing JSON: {e}"),
    }

    let json_str = fs::read_to_string(PATH).unwrap();
    let config_result = AsnLogConfig::from_json(json_str.as_str()).unwrap();
    println!("Parsed config: {config_result:?}");
}
