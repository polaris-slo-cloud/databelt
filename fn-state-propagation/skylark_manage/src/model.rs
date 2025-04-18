use std::fmt::{Display, Formatter};
// This is a part of Skylark.
// See README.md and LICENSE for details.
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct SkylarkKey {
    chain_id: String,
    node_id: String,
    task_id: String,
}

impl SkylarkKey {
    pub fn to_string(&self) -> String {
        format!("{}:{}:{}", self.chain_id, self.node_id, self.task_id)
    }

    pub fn chain_id(&self) -> &str {
        &self.chain_id
    }

    pub fn task_id(&self) -> &str {
        &self.task_id
    }

    pub fn set_chain_id(&mut self, chain_id: String) {
        self.chain_id = chain_id;
    }

    pub fn set_task_id(&mut self, task_id: String) {
        self.task_id = task_id;
    }
    pub fn valid(&self) -> bool {
        !self.chain_id.is_empty()
            && !self.task_id.is_empty()
            && !self.node_id.is_empty()
            && !self.chain_id.eq("unknown")
            && !self.chain_id.eq("unknown")
            && !self.node_id.eq("unknown")
    }

    pub fn new(chain_id: String, node_id: String, task_id: String) -> Self {
        Self {
            chain_id,
            node_id,
            task_id,
        }
    }

    pub fn node_id(&self) -> &str {
        &self.node_id
    }

    pub fn set_node_id(&mut self, node_id: String) {
        self.node_id = node_id;
    }
}
impl TryFrom<String> for SkylarkKey {
    type Error = ();

    fn try_from(item: String) -> Result<Self, Self::Error> {
        let mut split = item.split(':');
        let chain_id = split.next().ok_or(())?.to_string();
        let node_id = split.next().ok_or(())?.to_string();
        let task_id = split.next().ok_or(())?.to_string();
        Ok(SkylarkKey { chain_id, node_id, task_id })
    }
}

impl Default for SkylarkKey {
    fn default() -> Self {
        Self {
            task_id: "unknown".to_string(),
            chain_id: "unknown".to_string(),
            node_id: "unknown".to_string(),
        }
    }
}

impl PartialEq for SkylarkKey {
    fn eq(&self, other: &Self) -> bool {
        self.chain_id == other.chain_id && self.task_id == other.task_id && self.node_id == other.node_id
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
    Stateless,
}
impl From<String> for SkylarkPolicy {
    fn from(mode: String) -> Self {
        match mode.to_lowercase().as_str() {
            "skylark" => SkylarkPolicy::Skylark,
            "random" => SkylarkPolicy::Random,
            "stateless" => SkylarkPolicy::Stateless,
            _ => SkylarkPolicy::Skylark,
        }
    }
}

impl Display for SkylarkPolicy {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}
