use redis::{Client, Commands, RedisResult};
use reqwest::header::CONTENT_TYPE;
use crate::model::{SkylarkKey, SkylarkMode, SkylarkState};
use crate::LOCAL_NODE_HOST;
use crate::SKYLARK_API_PORT;

type Result<T> = std::result::Result<T, reqwest::Error>;
const LOCAL_REDIS_URL: &str = "redis://redis.default.svc.cluster.local:6379";

pub async fn get_skylark_state(key: &SkylarkKey) -> Result<SkylarkState> {
    let url = format!("http://{}:{}/{}", LOCAL_NODE_HOST.get().unwrap(), SKYLARK_API_PORT.get().unwrap(), key.to_string());
    info!("get_skylark_state: url: {}", url);
    reqwest::get(url).await?.json::<SkylarkState>().await
}

pub async fn get_local_state(key: &SkylarkKey) -> RedisResult<String> {
    let mut client = Client::open(LOCAL_REDIS_URL)?;
    let url = format!("http://{}:{}/{}", LOCAL_NODE_HOST.get().unwrap(), SKYLARK_API_PORT.get().unwrap(), key.to_string());
    info!("get_skylark_state: url: {}", url);
    info!(
        "get_local_state: Attempting to receive key: {}",
        key.to_string()
    );
    client.get(key.to_string())
}

pub async fn store_skylark_state(state: &SkylarkState, mode: &SkylarkMode) -> Result<String> {
    info!(
        "store_skylark_state: state: {}",
        state.value().clone()
    );
    let url = format!("http://{}:{}/save/{}", LOCAL_NODE_HOST.get().unwrap(), SKYLARK_API_PORT.get().unwrap(), mode.to_string().to_lowercase());
    debug!("store_skylark_state: url: {}", url);
    debug!("store_skylark_state: state: {}", serde_json::to_string(&state).unwrap());
    reqwest::Client::new()
        .post(url)
        .header(CONTENT_TYPE, "application/json")
        .json::<SkylarkState>(state)
        .send()
        .await?
        .text()
        .await
}

pub async fn delete_skylark_state(key: &SkylarkKey) -> Result<String> {
    let url = format!("http://{}:{}/{}", LOCAL_NODE_HOST.get().unwrap(), SKYLARK_API_PORT.get().unwrap(), key.to_string());
    info!("get_skylark_state: url: {}", url);
    reqwest::Client::new()
        .delete(url)
        .send()
        .await?
        .text()
        .await
}
