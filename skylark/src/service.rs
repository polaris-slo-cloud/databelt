use std::collections::HashMap;
use std::env;
use hyper::{Body, Client, Method, Request, Uri};
use crate::model::{Node, NodeGraph, PodInfo, SkylarkKey, SkylarkRedisMetadata, SkylarkSLOs, SkylarkState};
use redis::{AsyncCommands, RedisResult};
use serde::Serialize;
use serde_json::json;

const NODE_SERVICE_URL: &str = "http://skylark-neighbors.default.svc.cluster.local";
const LOCAL_REDIS_URL: &str = "redis://redis.default.svc.cluster.local:6379";
type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;
pub async fn get_node_graph() -> Result<NodeGraph> {
    info!("skylark::get_node_graph: init");
    let client = Client::new();
    let mut res = client.get(Uri::from(format!("{}/node-topology", {NODE_SERVICE_URL}))).await?;
    info!("skylark::get_node_graph: response status {}", res.status());
    let node_graph: NodeGraph = serde_json::from_str(res.into_body().try_into()?)?;
    Ok(node_graph)
}
pub async fn get_redis_metadata() -> Result<HashMap<Node, PodInfo>> {
    info!("skylark::get_redis_metadata: init");
    let client = Client::new();
    let mut res = client.get(Uri::from(format!("{}/redis-pods", {NODE_SERVICE_URL}))).await?;
    info!("skylark::get_redis_metadata: response status {}", res.status());
    let redis_metadata: HashMap<Node, PodInfo> = serde_json::from_str(res.into_body().try_into()?)?;
    Ok(redis_metadata)
}
pub async fn get_slos() -> Result<SkylarkSLOs> {
    info!("skylark::get_slo: init");
    let client = Client::new();
    let mut res = client.get(Uri::from(format!("{}/slo", {NODE_SERVICE_URL}))).await?;
    info!("skylark::get_slo: response status {}", res.status());
    let cluster_slos: SkylarkSLOs = serde_json::from_str(res.into_body().try_into().unwrap())?;
    Ok(cluster_slos)
}
pub async fn get_skylark_state(key: SkylarkKey, url: Option<String>) -> RedisResult<Option<SkylarkState>> {
    let url = url.unwrap_or_else(|| String::from(LOCAL_REDIS_URL));
    info!("get_skylark_state: Connecting to Redis at URL: {}", url);
    let client = redis::Client::open(url)?;
    let mut con = client.get_multiplexed_async_connection().await?;
    info!("get_skylark_state: Attempting to receive key: {}", key);
    let res_hash: HashMap<String, String> = con.hgetall(key).await?;
    match res_hash {
        Some(hash) => {
            info!("get_skylark_state: Received hashmap wih length: {}", hash.len());
            let json_string = serde_json::to_string(&hash).unwrap();
            let skylark_state: SkylarkState = serde_json::from_str(&json_string).unwrap();
            Ok(skylark_state)
        }
        None => {
            warn!("get_skylark_state: key not found");
            Ok(None)
        }
    }
}

pub async fn store_skylark_state(state: &SkylarkState, url: Option<String>) -> Result<()> {
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
        con.hset(state.get_key(), field, value).await?;
    }
    info!("store_skylark_state: SkylarkState successfully stored in Redis");
    Ok(())
}