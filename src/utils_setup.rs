use crate::asn_log_config::AsnLogConfig;
use std::sync::Once;
use crate::log::warn;

static INIT: Once = Once::new();

pub fn configure_logging(c: &AsnLogConfig) {
    INIT.call_once(|| {
        let mut builder = fern::Dispatch::new();
        let level_formatter;

        #[cfg(target_arch = "wasm32")]
        {
            level_formatter = |level| level;
            builder = builder.chain(fern::Output::call(console_log::log));
        }
        #[cfg(not(target_arch = "wasm32"))]
        {
            use fern::colors::{Color, ColoredLevelConfig};
            let colors = ColoredLevelConfig::new()
                .info(Color::Blue)
                .debug(Color::Green);
            level_formatter = move |level| colors.color(level);
            builder = builder.chain(std::io::stdout());
        }

        builder = builder
            .level(c.global_level.into())
            // .level_for(module_path!(), log::LevelFilter::Info)
            .format(move |out, message, record| {
                out.finish(format_args!(
                    "{}[{}][{}:{}] {}",
                    chrono::Local::now().format("[%H:%M:%S]"),
                    level_formatter(record.level()),
                    record.target(),
                    record.line().unwrap_or_default(),
                    message
                ))
            });

        for (module, level) in &c.module_levels {
            let m = module.clone();
            let l = (*level).into();
            builder = builder.level_for(m, l);
        }

        builder.apply().unwrap();
    });

    if INIT.is_completed() {
        warn!("Попытка повторной настройки логгера — игнорируется");
    }
}
