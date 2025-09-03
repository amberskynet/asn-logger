pub extern crate log;

mod asn_log_config;
mod asn_log_level;
mod macrodefines;
mod utils_setup;

pub use asn_log_config::AsnLogConfig;
pub use asn_log_level::AsnLogLevel;

// pub use log::{debug, error, info, trace, warn};
use utils_setup::configure_logging;

pub fn test_messages() {
    t_error!("asn-logger", "App error");
    t_debug!("asn-logger", "App debug");
    t_info!("asn-logger", "App info");
    t_warn!("asn-logger", "App warning");
    t_trace!("asn-logger", "App tracing");
}

pub fn init_log(c: &AsnLogConfig) -> Result<(), String> {
    cfg_if::cfg_if! {
    if #[cfg(target_arch = "wasm32")] {
        std::panic::set_hook(Box::new(console_error_panic_hook::hook));
            // выводим средствами браузера а не логгера, он еще не инициализирован
            web_sys::console::debug_1(&"console_error_panic_hook enabled".into());
        }
    }

    configure_logging(c)
}
