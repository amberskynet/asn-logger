extern crate asn_logger;

use asn_logger::{AsnLogConfig, AsnLogLevel, init_log};

const LOG_MODULE_NAME: &str = "main name";

mod module_2 {

    use asn_logger;

    const LOG_MODULE_NAME: &str = "module_2 name";

    pub fn look() {
        asn_logger::t_info!(LOG_MODULE_NAME, "Cool");
        asn_logger::m_info!("Cool");
        asn_logger::info!("Cool");
    }
}

mod module_1 {
    const LOG_MODULE_NAME: &str = "module_1 name";

    pub fn look() {
        asn_logger::t_info!(LOG_MODULE_NAME, "Cool");
        asn_logger::m_info!("Cool");
        asn_logger::info!("Cool");
    }
}

fn main() {
    let c = AsnLogConfig {
        global_level: AsnLogLevel::Trace,
        module_levels: Default::default(),
    };

    init_log(&c);

    module_1::look();
    module_2::look();

    asn_logger::t_info!(LOG_MODULE_NAME, "Cool");
    asn_logger::m_info!("Cool");
    asn_logger::info!("Cool");
}
