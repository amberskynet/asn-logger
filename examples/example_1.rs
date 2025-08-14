extern crate asn_logger;
use asn_logger::{AsnLogConfig, AsnLogLevel, init_log};

#[cfg(not(feature = "test_messages"))]
fn main() {
    panic!("This example requires 'test_messages' feature!");
}

#[cfg(feature = "test_messages")]
fn main() {
    let main_module_name = "main()";

    let mut c = AsnLogConfig {
        global_level: AsnLogLevel::Trace,
        module_levels: Default::default(),
    };

    c.module_levels
        .insert(String::from(main_module_name), AsnLogLevel::Warn);

    init_log(&c);

    asn_logger::t_warn!(main_module_name, "Say Warn");
    asn_logger::t_trace!(main_module_name, "Say Trace"); // don't see this if set AsnLogLevel::Warn to main_module_name
}
