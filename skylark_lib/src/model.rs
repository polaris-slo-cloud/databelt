// This is a part of Skylark.
// See README.md and LICENSE for details.
use serde::{Deserialize, Serialize};


#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct SkylarkKey {
    chain_id: String,
    fn_name: String,
}

impl SkylarkKey {
    pub fn to_string(&self) -> String {
        format!("{}:{}", self.chain_id, self.fn_name)
    }

    pub fn new(chain_id: String, fn_name: String) -> Self {
        Self { chain_id, fn_name }
    }

    pub fn chain_id(&self) -> &str {
        &self.chain_id
    }

    pub fn fn_name(&self) -> &str {
        &self.fn_name
    }

    pub fn set_chain_id(&mut self, chain_id: String) {
        self.chain_id = chain_id;
    }

    pub fn set_fn_name(&mut self, fn_name: String) {
        self.fn_name = fn_name;
    }
    pub fn valid(&self) -> bool {
        !self.chain_id.is_empty() && !self.fn_name.is_empty()
    }
}
impl From<String> for SkylarkKey {
    fn from(item: String) -> Self {
        let mut split = item.split(':');
        let chain_id = split.next().unwrap().to_string();
        let fn_name = split.next().unwrap().to_string();
        SkylarkKey { chain_id, fn_name }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SkylarkState {
    key: SkylarkKey,
    value: String,
}

impl SkylarkState {
    pub fn to_string(&self) -> String {
        format!(
            "SkylarkState\n\tSkylarkKey: {}\n\tValue: {}\n",
            self.key.to_string(),
            self.value.to_string()
        )
    }

    pub fn new(key: SkylarkKey, value: String) -> Self {
        Self { key, value }
    }

    pub fn key(&self) -> &SkylarkKey {
        &self.key
    }

    pub fn value(&self) -> &String {
        &self.value
    }

    pub fn set_key(&mut self, key: SkylarkKey) {
        self.key = key;
    }

    pub fn set_value(&mut self, value: String) {
        self.value = value;
    }
}

#[derive(Serialize, Deserialize)]
pub enum SkylarkMode {
    Cloud,
    Edge,
    Sat,
}
impl From<String> for SkylarkMode {
    fn from(mode: String) -> Self {
        match mode.as_ref() {
            "satellite" => SkylarkMode::Sat,
            "sat" => SkylarkMode::Sat,
            "edge" => SkylarkMode::Edge,
            _ => SkylarkMode::Cloud,
        }
    }
}
