use redis::{Client, Commands, RedisResult};
use crate::model::{SkylarkKey, SkylarkMode, SkylarkState};
use crate::LOCAL_NODE_HOST;
use crate::SKYLARK_API_PORT;

type Result<T> = std::result::Result<T, reqwest::Error>;
const LOCAL_REDIS_URL: &str = "redis://redis.default.svc.cluster.local:6379";

pub async fn get_skylark_state(key: &SkylarkKey) -> Result<SkylarkState> {
    let url = format!("http://{}:{}/{}", LOCAL_NODE_HOST.get().unwrap(), SKYLARK_API_PORT.get().unwrap(), key.to_string());
    info!("skylark::get_skylark_state: url: {}", url);
    reqwest::get(url).await?.json::<SkylarkState>().await
}

pub async fn get_local_state(key: &SkylarkKey) -> RedisResult<String> {
    let mut client = Client::open(LOCAL_REDIS_URL)?;
    let url = format!("http://{}:{}/{}", LOCAL_NODE_HOST.get().unwrap(), SKYLARK_API_PORT.get().unwrap(), key.to_string());
    info!("skylark::get_skylark_state: url: {}", url);
    info!(
        "get_local_state: Attempting to receive key: {}",
        key.to_string()
    );
    client.get(key.to_string())
}

pub async fn store_skylark_state(state: &SkylarkState, mode: &SkylarkMode) -> Result<SkylarkState> {
    info!(
        "skylark::store_skylark_state: state: {}",
        state.value().clone()
    );
    let url = format!("http://{}:{}/save/{}", LOCAL_NODE_HOST.get().unwrap(), SKYLARK_API_PORT.get().unwrap(), mode.to_string().to_lowercase());
    reqwest::Client::new()
        .post(url)
        .json::<SkylarkState>(state)
        .send()
        .await?
        .json::<SkylarkState>()
        .await
}

pub async fn delete_skylark_state(key: &SkylarkKey) -> Result<()> {
    let url = format!("http://{}:{}/{}", LOCAL_NODE_HOST.get().unwrap(), SKYLARK_API_PORT.get().unwrap(), key.to_string());
    info!("skylark::get_skylark_state: url: {}", url);
    reqwest::Client::new()
        .delete(url)
        .send()
        .await?
        .json::<()>()
        .await
}
