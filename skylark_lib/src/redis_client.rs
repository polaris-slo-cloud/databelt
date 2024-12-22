use crate::model::{SkylarkState};
use redis::{AsyncCommands, Client, RedisResult};

pub async fn store_state_by_host(state: &SkylarkState, host: &str) -> RedisResult<()> {
    debug!(
        "store_state_by_host: Attempting to store key {} at redis host {}",
        state.key().to_string(),
        host
    );
    let client = Client::open(format!("redis://{}:6379/", host))?;
    let mut con = client.get_multiplexed_async_connection().await?;
    con.set(state.key().to_string(), state.value()).await
}
pub async fn get_state_by_host(key: &String, host: &str) -> RedisResult<String> {
    debug!(
        "get_state_by_host: Attempting to receive key {} from host {}",
        key.to_string(),
        host
    );
    let client = Client::open(format!("redis://{}:6379/", host))?;
    let mut con = client.get_multiplexed_async_connection().await?;
    con.get(key.to_string()).await
}
pub async fn del_state_by_host(key: &String, host: &str) -> RedisResult<()> {
    debug!(
        "del_state_by_host: Attempting to receive key {} from host {}",
        key.to_string(),
        host
    );
    let client = Client::open(format!("redis://{}:6379/", host))?;
    let mut con = client.get_multiplexed_async_connection().await?;
    con.del(key.to_string()).await
}