use std::fmt::{Display, Formatter};
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
        !self.chain_id.is_empty() && !self.fn_name.is_empty() && !self.chain_id.eq("unknown") && !self.chain_id.eq("unknown")
    }
}
impl TryFrom<String> for SkylarkKey {
    type Error = ();

    fn try_from(item: String) -> Result<Self, Self::Error> {
        let mut split = item.split(':');
        let chain_id = split.next().ok_or(())?.to_string();
        let fn_name = split.next().ok_or(())?.to_string();
        Ok(SkylarkKey { chain_id, fn_name })
    }
}

impl Default for SkylarkKey {
    fn default() -> Self {
        Self {
            fn_name: "unknown".to_string(),
            chain_id: "unknown".to_string(),
        }
    }
}

impl PartialEq for SkylarkKey {
    fn eq(&self, other: &Self) -> bool {
        self.chain_id == other.chain_id && self.fn_name == other.fn_name
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
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

impl Default for SkylarkState {
    fn default() -> Self {
        Self {
            key: SkylarkKey::default(),
            value: "unknown".to_string(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct SkylarkBundledState {
    key: SkylarkKey,
    fields: Vec<(String, String)>,
}

impl SkylarkBundledState {
    pub fn new(key: SkylarkKey, fields: Vec<(String, String)>) -> Self {
        Self { key, fields }
    }

    pub fn key(&self) -> &SkylarkKey {
        &self.key
    }

    pub fn fields(&self) -> &Vec<(String, String)> {
        &self.fields
    }

    pub fn set_key(&mut self, key: SkylarkKey) {
        self.key = key;
    }

    pub fn set_fields(&mut self, fields: Vec<(String, String)>) {
        self.fields = fields;
    }
}

impl Default for SkylarkBundledState {
    fn default() -> Self {
        Self {
            key: SkylarkKey::default(),
            fields: Vec::new(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum SkylarkPolicy {
    Skylark,
    Random,
    Serverless
}
impl From<String> for SkylarkPolicy {
    fn from(mode: String) -> Self {
        match mode.to_lowercase().as_str() {
            "skylark" => SkylarkPolicy::Skylark,
            "random" => SkylarkPolicy::Random,
            "serverless" => SkylarkPolicy::Serverless,
            _ => SkylarkPolicy::Skylark,
        }
    }
}

impl Display for SkylarkPolicy {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum SkylarkStorageType {
    Single,
    Bundled
}
impl From<String> for SkylarkStorageType {
    fn from(mode: String) -> Self {
        match mode.to_lowercase().as_str() {
            "single" => SkylarkStorageType::Single,
            "bundled" => SkylarkStorageType::Bundled,
            _ => SkylarkStorageType::Single,
        }
    }
}

impl Display for SkylarkStorageType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}
