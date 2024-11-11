use hyper::{Body, Client, Method, Request, Uri};
use serde::Deserialize;
use serde::Serialize;
use std::collections::HashMap;
use std::env;
use std::sync::Mutex;
use hyper::body::HttpBody;
type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

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

pub async fn get_nodes() -> Result<()> {
    println!("skylark::get_nodes: init");
    let url = "http://10.152.183.152/neighbors"
        .parse::<Uri>()
        .expect("Invalid URI");
    println!("skylark::get_nodes: url {}", url);
    let client = Client::new();
    let req = Request::builder()
        .method(Method::GET)
        .header(hyper::header::HOST, hyper::header::HeaderValue::from_static("skylark-neighbors.default.svc.cluster.local"))
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
