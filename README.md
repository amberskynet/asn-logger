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
```
