use serde::{Deserialize, Serialize};

use crate::AsnLogLevel;
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
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

// impl AsnLogConfig {
//     pub fn from_json(json_str: &str) -> Result<Self, serde_json::Error> {
//         serde_json::from_str(json_str)
//     }

//     pub fn to_json(&self) -> Result<String, serde_json::Error> {
//         serde_json::to_string_pretty(self)
//     }
// }
