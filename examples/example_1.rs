extern crate asn_logger;
use asn_logger::{init_log, AsnLogConfig, AsnLogLevel};

#[cfg(not(feature = "test_messages"))]
fn main() {
    panic!("This example requires 'test_messages' feature!");
}

#[cfg(feature = "test_messages")]
fn main() {
    let c = AsnLogConfig {
        global_level: AsnLogLevel::Trace,
        module_levels: Default::default(),
    };

    init_log(&c);
}
