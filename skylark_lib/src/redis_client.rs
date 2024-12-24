use crate::model::{SkylarkBundledState, SkylarkState, SkylarkStorageType};
use crate::SkylarkKey;
use redis::{AsyncCommands, Client, RedisResult};

pub async fn set_single_state_by_host(
    state: &SkylarkState,
    host: &str,
    storage_type: &SkylarkStorageType,
) -> RedisResult<()> {
    debug!(
        "set_state_by_host: Attempting to store key {} at redis host {} and storage type {:?}",
        state.key().to_string(),
        host,
        storage_type
    );
    let client = Client::open(format!("redis://{}:6379/", host))?;
    let mut con = client.get_multiplexed_async_connection().await?;
    match storage_type {
        SkylarkStorageType::Bundled => {
            con.hset(
                state.key().chain_id(),
                state.key().fn_name(),
                state.value().to_string(),
            )
            .await
        }
        SkylarkStorageType::Single => con.set(state.key().to_string(), state.value()).await
    }
}

pub async fn set_bundled_state_by_host(
    state: &SkylarkBundledState,
    host: &str,
    storage_type: &SkylarkStorageType,
) -> RedisResult<()> {
    debug!(
        "set_bundled_state_by_host: Attempting to store key {} at redis host {} and storage type {:?}",
        state.key().chain_id(),
        host,
        storage_type
    );
    let client = Client::open(format!("redis://{}:6379/", host))?;
    let mut con = client.get_multiplexed_async_connection().await?;
    con.hset_multiple(state.key().chain_id(), state.fields())
        .await
}
pub async fn get_single_state_by_host(
    key: &SkylarkKey,
    host: &str,
    storage_type: &SkylarkStorageType,
) -> RedisResult<String> {
    debug!(
        "get_single_state_by_host: Attempting to get key {} from host {} and storage type {:?}",
        key.to_string(),
        host,
        storage_type
    );
    let client = Client::open(format!("redis://{}:6379/", host))?;
    let mut con = client.get_multiplexed_async_connection().await?;
    match storage_type {
        SkylarkStorageType::Bundled => con.hget(key.chain_id(), key.fn_name()).await,
        SkylarkStorageType::Single => con.get(key.to_string()).await
    }
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
pub async fn del_state_by_host(
    key: &SkylarkKey,
    host: &str,
    storage_type: &SkylarkStorageType,
) -> RedisResult<()> {
    debug!(
        "del_state_by_host: Attempting to delete key {} from host {} and storage type {:?}",
        key.to_string(),
        host,
        storage_type
    );
    let client = Client::open(format!("redis://{}:6379/", host))?;
    let mut con = client.get_multiplexed_async_connection().await?;
    match storage_type {
        SkylarkStorageType::Bundled => con.del(key.chain_id()).await,
        SkylarkStorageType::Single => con.del(key.to_string()).await
    }
}
