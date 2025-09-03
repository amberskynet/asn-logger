extern crate asn_logger;

use asn_logger::{AsnLogConfig, AsnLogLevel, init_log};

const LOG_MODULE_NAME: &str = "main name";

mod module_2 {

    use asn_logger;

    const LOG_MODULE_NAME: &str = "module_2 name";

    pub fn look() {
        asn_logger::t_info!("Target module 2", "Cool");
        asn_logger::m_info!("Cool");
        asn_logger::log::info!("Cool");
    }
}

mod module_1 {
    const LOG_MODULE_NAME: &str = "module_1 name";

    pub fn look() {
        asn_logger::t_info!("Target module 1", "Cool");
        asn_logger::m_info!("Cool");
        asn_logger::log::info!("Cool");
    }
}

fn main() {
    let c = AsnLogConfig {
        global_level: AsnLogLevel::Trace,
        module_levels: Default::default(),
    };

    init_log(&c).unwrap();

    module_1::look();
    module_2::look();

    asn_logger::t_info!("Target main", "Cool");
    asn_logger::m_info!("Cool");
    asn_logger::log::info!("Cool");
}
