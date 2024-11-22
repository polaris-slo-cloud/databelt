use std::collections::HashMap;
use std::env;
use std::string::ToString;
use crate::model::{SkylarkKey, SkylarkState};
use redis::{AsyncCommands};
use serde::de::DeserializeOwned;

static NODE_SERVICE_URL: &str = "http://skylark-neighbors.default.svc.cluster.local";
static LOCAL_REDIS_URL: &str = "redis://redis.default.svc.cluster.local:6379";
type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

pub async fn get_from_node_provider<T>(path: &str) -> Result<T>
where
    T: DeserializeOwned,
{
    info!("skylark::get_from_node_provider: init");
    let node_url = format!("{}/{}",env::var("NODE_PROVIDER_URL").unwrap_or_else(|_| NODE_SERVICE_URL.parse().unwrap()), path);
    info!("skylark::get_from_node_provider: node_url: {}", node_url);
    let response = reqwest::get(node_url).await;

    match response {
        Ok(res) => {
            match res.status().is_success() {
                true => {
                    let json : Result<T> = res.json().await?;
                    match json {
                        Ok(json_data) => {
                            Ok(json_data)
                        },
                        Err(e) => {
                            Err(Box::new(e))
                        },
                    }
                },
                false => {
                    Err(Box::new(std::io::Error::new(
                        std::io::ErrorKind::Other,
                        format!("Request failed with status: {}", res.status()),
                    )))
                },
            }
        },
        Err(err) => {
            Err(Box::new(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!("Request failed : {}", err.to_string()),
            )))
        },
    }
}

pub async fn get_skylark_state(key: &SkylarkKey, url: Option<String>) -> Result<SkylarkState> {
    let url = url.unwrap_or_else(|| String::from(LOCAL_REDIS_URL));
    info!("get_skylark_state: Connecting to Redis at URL: {}", url);
    let client = redis::Client::open(url)?;
    let mut con = client.get_multiplexed_async_connection().await?;
    info!("get_skylark_state: Attempting to receive key: {}", key);
    let res_hash = con.hgetall(key.to_string())?;
    let json_string = serde_json::to_string(&res_hash)?;
    serde_json::from_str(&json_string)?
}

pub async fn store_skylark_state(state: &SkylarkState, url: Option<String>) -> Result<String> {
    let url = url.unwrap_or_else(|| String::from(LOCAL_REDIS_URL));
    info!("store_skylark_state: Connecting to Redis at URL: {}", url);
    let client = redis::Client::open(url)?;
    let mut con = client.get_multiplexed_async_connection().await?;
    info!("store_skylark_state: Attempting to store key: {}", state.get_key());
    let value_map: HashMap<String, String> = serde_json::to_value(state.get_data())?
        .as_object()
        .ok_or_else(|| redis::RedisError::from((redis::ErrorKind::TypeError, "Serialization error")))?
        .iter()
        .map(|(k, v)| (k.clone(), v.to_string()))
        .collect();
    for (field, value) in value_map {
        con.hset(state.get_key(), field, value);
    }
    Ok(state.get_key())
}