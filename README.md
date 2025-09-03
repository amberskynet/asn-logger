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

### Явное указание target

```Rust
    let target = "main()";
...
    asn_logger::t_warn!(target, "Say Warn");
```

### target берется из константы LOG_MODULE_NAME

```Rust
    const LOG_MODULE_NAME: &str = "Cool name";
...
    asn_logger::m_info!("Cool");
