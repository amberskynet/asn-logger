# ASN Logger

ASN Logger - это библиотека для логирования в Rust с поддержкой различных уровней логирования, настроек и возможностью работы в различных средах, включая WebAssembly.

## Лицензия

Этот проект распространяется под лицензией GNU General Public License v3.0. Подробности см. в файле [LICENSE](LICENSE).

## Установка

Чтобы использовать asn-logger вашем проекте, добавьте следующую строку в раздел `[dependencies]` вашего `Cargo.toml`:

```toml
[dependencies]
asn-logger = "3.0"
```

### features

`test_messages` - выводит отладочные сообщения всех видов в консоль сразу после инициализации
для проверки корректности отображения ошибок

## Варианты использования

### 1. Базовая настройка логгера

Для начала работы с asn-logger необходимо инициализировать его с конфигурацией:

```rust
use asn_logger::{AsnLogConfig, AsnLogLevel, init_log};

fn main() {
    let config = AsnLogConfig::new(AsnLogLevel::Trace);
    init_log(&config).expect("Failed to initialize logger");
    
    // Теперь можно использовать макросы логирования
    asn_logger::t_info!("main", "Logger initialized successfully");
}
```

### 2. Настройка различных уровней логирования для модулей

Можно задать разные уровни логирования для разных модулей:

```rust
use asn_logger::{AsnLogConfig, AsnLogLevel, init_log};
use std::collections::HashMap;

fn main() {
    let mut config = AsnLogConfig::new(AsnLogLevel::Info);
    config.set_module_level("network", AsnLogLevel::Debug);
    config.set_module_level("database", AsnLogLevel::Warn);
    
    init_log(&config).expect("Failed to initialize logger");
    
    // Будут отображаться только сообщения уровня Info и выше
    asn_logger::t_trace!("main", "This won't be shown");
    asn_logger::t_info!("main", "This will be shown");
    
    // Для модуля network будут отображаться сообщения уровня Debug и выше
    asn_logger::t_debug!("network", "Debug message from network module");
    
    // Для модуля database будут отображаться только сообщения уровня Warn и выше
    asn_logger::t_info!("database", "This won't be shown");
    asn_logger::t_warn!("database", "This will be shown");
}
```

### 3. Использование макросов логирования с явным указанием target

Макросы с префиксом `t_` позволяют явно указать target (модуль) для каждого сообщения:

```rust
// Примеры использования макросов с явным указанием target
asn_logger::t_error!("network", "Connection failed: {}", error_message);
asn_logger::t_warn!("config", "Deprecated setting used: {}", setting_name);
asn_logger::t_info!("main", "Application started successfully");
asn_logger::t_debug!("database", "Query executed in {} ms", execution_time);
asn_logger::t_trace!("parser", "Parsing token: {:?}", token);
```

### 4. Использование макросов логирования с константой LOG_MODULE_NAME

Макросы с префиксом `m_` используют константу `LOG_MODULE_NAME`, определенную в текущем модуле:

```rust
const LOG_MODULE_NAME: &str = "auth";

fn authenticate_user(username: &str) -> Result<(), String> {
    asn_logger::m_debug!("Attempting to authenticate user: {}", username);
    
    // Логика аутентификации...
    
    asn_logger::m_info!("User {} authenticated successfully", username);
    Ok(())
}
```

### 5. Конфигурация логгера из JSON

asn-logger поддерживает настройку через JSON конфигурацию:

```json
{
  "global_level": "Info",
  "module_levels": {
    "network": "Debug",
    "database": "Warn"
  }
}
```

Загрузка конфигурации из JSON строки:

```rust
use asn_logger::AsnLogConfig;
use serde_json;

fn main() {
    let json_str = r#"
        {
            "global_level": "Info",
            "module_levels": {
                "network": "Debug",
                "database": "Warn"
            }
        }
    "#;
    
    let config: AsnLogConfig = serde_json::from_str(json_str)
        .expect("Failed to parse logger configuration");
        
    asn_logger::init_log(&config).expect("Failed to initialize logger");
}
```

### 6. Использование WebAssembly

asn-logger также поддерживает работу в среде WebAssembly, что позволяет использовать
единый подход к логированию как в нативных, так и в веб-приложениях.
