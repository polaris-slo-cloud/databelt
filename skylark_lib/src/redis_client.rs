use crate::model::{SkylarkBundledState, SkylarkState};
use crate::SkylarkKey;
use redis::{AsyncCommands, Client, RedisResult};

pub async fn set_single_state_by_host(state: &SkylarkState, host: &str) -> RedisResult<()> {
    debug!(
        "set_state_by_host: Attempting to store key {} at redis host {}",
        state.key().to_string(),
        host
    );
    let client = Client::open(format!("redis://{}:6379/", host))?;
    let mut con = client.get_multiplexed_async_connection().await?;
    con.set(state.key().to_string(), state.value()).await
}

pub async fn set_bundled_state_by_host(state: &SkylarkBundledState, host: &str) -> RedisResult<()> {
    debug!(
        "set_bundled_state_by_host: Attempting to store key {} at redis host {}",
        state.key().chain_id(),
        host
    );
    let client = Client::open(format!("redis://{}:6379/", host))?;
    let mut con = client.get_multiplexed_async_connection().await?;
    con.hset_multiple(state.key().chain_id(), state.fields())
        .await
}
pub async fn get_single_state_by_host(key: &SkylarkKey, host: &str) -> RedisResult<String> {
    debug!(
        "get_single_state_by_host: Attempting to get key {} from host {}",
        key.to_string(),
        host
    );
    let client = Client::open(format!("redis://{}:6379/", host))?;
    let mut con = client.get_multiplexed_async_connection().await?;
    con.get(key.to_string()).await
}

pub async fn get_bundled_state_by_host(
    key: &SkylarkKey,
    host: &str,
) -> RedisResult<Vec<(String, String)>> {
    debug!(
        "get_bundled_state_by_host: Attempting to get key {} from host {}",
        key.to_string(),
        host
    );
    let client = Client::open(format!("redis://{}:6379/", host))?;
    let mut con = client.get_multiplexed_async_connection().await?;
    con.hgetall(key.chain_id()).await
}
pub async fn del_single_state_by_host(key: &SkylarkKey, host: &str) -> RedisResult<()> {
    debug!(
        "del_single_state_by_host: Attempting to delete key {} from host {}",
        key.to_string(),
        host
    );
    let client = Client::open(format!("redis://{}:6379/", host))?;
    let mut con = client.get_multiplexed_async_connection().await?;
    con.del(key.to_string()).await
}
pub async fn del_bundled_state_by_host(key: &SkylarkKey, host: &str) -> RedisResult<()> {
    debug!(
        "del_bundled_state_by_host: Attempting to delete key {} from host {}",
        key.to_string(),
        host
    );
    let client = Client::open(format!("redis://{}:6379/", host))?;
    let mut con = client.get_multiplexed_async_connection().await?;
    con.del(key.chain_id()).await
}
