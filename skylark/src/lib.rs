use hyper::body::Buf;
use hyper::{Client, Uri};
use serde::Deserialize;
use serde::Serialize;
use std::collections::HashMap;
use std::env;
use std::error::Error;
use std::sync::Mutex;
pub struct State {
    data: Mutex<HashMap<String, String>>,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct NodeInfo {
    name: String,
    host: String,
}

impl State {
    pub fn new() -> Self {
        State {
            data: Mutex::new(HashMap::new()),
        }
    }

    pub fn set(&self, key: String, value: String) {
        let mut data = self.data.lock().unwrap();
        data.insert(key, value);
    }

    pub fn get(&self, key: &str) -> Option<String> {
        let data = self.data.lock().unwrap();
        data.get(key).cloned()
    }
}

pub async fn get_nodes() -> Result<Vec<NodeInfo>, Box<dyn Error>> {
    println!("skylark::get_nodes: init");
    let url = "http://skylark-neighbors.default.svc.cluster.local:8080/neighbors"
        .parse::<Uri>()
        .expect("Invalid URI");
    println!("skylark::get_nodes: url {}", url);
    let client = Client::new();
    let res = client.get(url).await?;
    println!("skylark::get_nodes: res status {}", res.status());
    if res.status() != 200 {
        return Err(format!("Received non-200 response: {}", res.status()).into());
    }
    let body = hyper::body::aggregate(res).await?;
    let node_info: Vec<NodeInfo> = serde_json::from_reader(body.reader())?;
    println!("skylark::get_nodes: node_info {:?}", node_info);
    Ok(node_info)
}

pub fn get_version() -> &'static str {
    env!("CARGO_PKG_VERSION")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn set_and_get_key() {
        let state = State::new();
        state.set("key1".to_string(), "value1".to_string());
        assert_eq!(state.get("key1"), Some("value1".to_string()));
    }
}
