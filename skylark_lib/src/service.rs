use crate::model::{SkylarkKey, SkylarkMode, SkylarkState};
use crate::LOCAL_REDIS_URL;
use crate::SKYLARK_API_URL;
use redis::{Client, Commands, RedisResult};
use reqwest::header::CONTENT_TYPE;

type Result<T> = std::result::Result<T, reqwest::Error>;

pub async fn get_skylark_state(key: &SkylarkKey, mode: SkylarkMode) -> Result<String> {
    let url = format!(
        "{}/state?key={}&mode={}",
        SKYLARK_API_URL.get().unwrap(),
        key.to_string(),
        mode.to_string()
    );
    debug!("get_skylark_state: url: {}", url);
    reqwest::get(url).await?.text().await
}

pub async fn get_local_state(key: &SkylarkKey) -> RedisResult<String> {
    let mut client = Client::open(LOCAL_REDIS_URL.get().unwrap().to_string())?;
    debug!(
        "get_local_state: Attempting to receive key from local KV store: {}",
        key.to_string()
    );
    client.get(key.to_string())
}

pub async fn store_skylark_state(state: &SkylarkState, mode: &SkylarkMode) -> Result<String> {
    debug!("store_skylark_state: state: {}", state.value().clone());
    let url = format!(
        "{}/save/{}",
        SKYLARK_API_URL.get().unwrap(),
        mode.to_string().to_lowercase()
    );
    debug!("store_skylark_state: url: {}", url);
    reqwest::Client::new()
        .post(url)
        .header(CONTENT_TYPE, "application/json")
        .json::<SkylarkState>(state)
        .send()
        .await?
        .text()
        .await
}

pub async fn delete_skylark_state(key: &SkylarkKey, mode: &SkylarkMode) -> Result<String> {
    let url = format!(
        "{}/state?key={}&mode={}",
        SKYLARK_API_URL.get().unwrap(),
        key.to_string(),
        mode.to_string()
    );
    debug!("delete_skylark_state: url: {}", url);
    reqwest::Client::new()
        .delete(url)
        .send()
        .await?
        .text()
        .await
}
