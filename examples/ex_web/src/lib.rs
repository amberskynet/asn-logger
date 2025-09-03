use asn_logger::{log::info, test_messages};
mod log_utils;

pub const LOG_MODULE_NAME: &str = "ex_web";

use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::spawn_local;

#[wasm_bindgen]
pub fn init_web_app() -> Result<(), JsValue> {
    if let Err(e) = log_utils::setup_log() {
        return Err(e.into());
    }

    test_messages();

    // Простая инициализация для web
    info!("ASN Web App initialized");

    // Здесь можно добавить базовую логику без проблемных зависимостей
    spawn_local(async move {
        info!("Async task started");

        // Простая асинхронная задача без задержки
        info!("Async task completed");
    });

    Ok(())
}

#[wasm_bindgen]
pub fn greet(name: &str) -> String {
    format!("Hello, {}! Welcome to ASN Web!", name)
}

#[wasm_bindgen]
pub fn get_version() -> String {
    "ASN Web WGPU v0.1.0".to_string()
}
