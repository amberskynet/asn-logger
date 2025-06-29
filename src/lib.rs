extern crate log;
extern crate serde_json;

mod asn_log_config;
mod asn_log_level;
mod utils_setup;

pub use asn_log_config::AsnLogConfig;
pub use asn_log_level::AsnLogLevel;

use log::{debug, error, info, trace, warn};
use utils_setup::configure_logging;

pub fn init_log_from_json(json_str: &str) -> Result<(), String> {
    let config_result = AsnLogConfig::from_json(json_str);

    match config_result {
        Ok(conf) => {
            init_log(&conf);
            Ok(())
        }
        Err(e) => Err(e.to_string()),
    }
}

pub fn init_log(c: &AsnLogConfig) {
    configure_logging(c);

    cfg_if::cfg_if! {
    if #[cfg(target_arch = "wasm32")] {
        std::panic::set_hook(Box::new(console_error_panic_hook::hook));
            #[cfg(feature = "test_messages")]
            debug("asn-logger", "console_error_panic_hook enabled");
        }
    }

    #[cfg(feature = "test_messages")]
    {
        error("asn-logger", "App error");
        debug("asn-logger", "App debug");
        info("asn-logger", "App info");
        warn("asn-logger", "App warning");
        trace("asn-logger", "App tracing");
    }
}

pub fn trace(t: &str, s: &str) {
    trace!(target: t, "{s}");
}

pub fn warn(t: &str, s: &str) {
    warn!(target: t, "{s}");
}

pub fn info(t: &str, s: &str) {
    info!(target: t, "{s}");
}

pub fn debug(t: &str, s: &str) {
    debug!(target: t, "{s}");
}

pub fn error(t: &str, s: &str) {
    error!(target: t, "{s}");
}
