extern crate asn_logger;

use asn_logger::AsnLogConfig;

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

    let config_result: Result<AsnLogConfig, serde_json::Error> = serde_json::from_str(json_str);

    match config_result {
        Ok(config) => {
            println!("Parsed config: {config:?}");
        }
        Err(e) => eprintln!("Error parsing JSON: {e}"),
    }
}
