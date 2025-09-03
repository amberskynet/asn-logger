use crate::asn_log_config::AsnLogConfig;

pub fn configure_logging(c: &AsnLogConfig) -> Result<(), String> {
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

    if let Err(e) = builder.apply() {
        return Err(e.to_string());
    }

    Ok(())
}
