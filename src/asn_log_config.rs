use crate::AsnLogLevel;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct AsnLogConfig {
    pub global_level: AsnLogLevel,
    pub module_levels: HashMap<String, AsnLogLevel>,
}

impl AsnLogConfig {
    pub fn new(global_level: AsnLogLevel) -> Self {
        Self {
            global_level,
            module_levels: HashMap::new(),
        }
    }

    pub fn set_module_level(&mut self, module: impl Into<String>, level: AsnLogLevel) {
        self.module_levels.insert(module.into(), level);
    }
}
