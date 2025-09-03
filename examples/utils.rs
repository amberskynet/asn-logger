use asn_logger::{AsnLogConfig, init_log};

pub fn init_log_from_json(json_str: &str) -> Result<(), String> {
    let config_result: Result<AsnLogConfig, serde_json::Error> = serde_json::from_str(json_str);

    match config_result {
        Ok(conf) => {
            if let Err(e) = init_log(&conf) {
                return Err(e);
            }
            Ok(())
        }
        Err(e) => Err(e.to_string()),
    }
}

#[allow(dead_code)]
fn main() {}
