use hyper::{Body, Client, Method, Request, Uri};
use serde::Deserialize;
use serde::Serialize;
use std::collections::HashMap;
use std::env;
use std::sync::Mutex;
use hyper::body::HttpBody;
extern crate pretty_env_logger;
#[macro_use]
extern crate log;
type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

pub struct State {
    data: Mutex<HashMap<String, String>>,
}
#[derive(Serialize, Deserialize, Debug)]
struct Edge {
    bandwidth: String,
    latency: String,
    source: String,
    target: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct Node {
    name: String,
    redis_pod_name: String,
    status: String,
    node_type: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct NetworkTopology {
    edges: Vec<Edge>,
    nodes: Vec<Node>,
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

pub async fn get_nodes() -> Result<()> {
    println!("skylark::get_nodes: init");
    let url = "http://skylark-neighbors.default.svc.cluster.local/node-topology"
        .parse::<Uri>()
        .expect("Invalid URI");
    println!("skylark::get_nodes: url {}", url);
    let client = Client::new();
    let req = Request::builder()
        .method(Method::GET)
        .uri(url)
        .body(Body::empty())
        .expect("Unable to build hyper::Request");

    let mut res = client.request(req).await?;
    let mut resp_data = Vec::new();
    while let Some(next) = res.data().await {
        let chunk = next?;
        resp_data.extend_from_slice(&chunk);
    }
    println!("{}", String::from_utf8_lossy(&resp_data));

    Ok(())
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
