extern crate asn_logger;

use asn_logger::init_log_from_json;
use std::fs;

const PATH: &str = "./examples/log_config.json";

fn main() {
    let main_module_name = "main()";

    let json_str = fs::read_to_string(PATH).unwrap();

    init_log_from_json(json_str.as_str()).unwrap();

    asn_logger::t_trace!(main_module_name, "Say trace");
    asn_logger::t_warn!(main_module_name, "Say warn");
}
