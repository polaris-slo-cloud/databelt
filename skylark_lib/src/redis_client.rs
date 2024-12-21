use crate::model::{SkylarkKey, SkylarkState};
use crate::{CLOUD_NODE, LOCAL_NODE};
use lazy_static::lazy_static;
use redis::{AsyncCommands, Client, Commands, RedisResult};
use std::sync::Mutex;

lazy_static! {
    static ref LOCAL_REDIS_CLIENT: Mutex<Client> = {
        let local_node = LOCAL_NODE.lock().unwrap();
        debug!(
            "Creating a local redis client with host {}",
            local_node.redis_host()
        );
        let client = Client::open(local_node.redis_host()).unwrap();
        Mutex::new(client)
    };
    static ref GLOBAL_REDIS_CLIENT: Mutex<Client> = {
        let cloud_node = CLOUD_NODE.lock().unwrap();
        debug!(
            "Creating a cloud redis client with host {}",
            cloud_node.redis_host()
        );
        let client = Client::open(cloud_node.redis_host()).unwrap();
        Mutex::new(client)
    };
}

pub fn connect_global() -> Result<redis::Connection, redis::RedisError> {
    debug!("Getting global redis connection");
    GLOBAL_REDIS_CLIENT.lock().unwrap().get_connection()
}
pub fn connect_local() -> Result<redis::Connection, redis::RedisError> {
    debug!("Getting local redis connection");
    LOCAL_REDIS_CLIENT.lock().unwrap().get_connection()
}
#[allow(dead_code)]
pub async fn get_local_state(key: &SkylarkKey) -> RedisResult<String> {
    debug!(
        "get_local_state: Attempting to receive key: {}",
        key.to_string()
    );
    let mut con = connect_local()?;
    con.get(key.to_string())
}

pub async fn store_local_state(state: &SkylarkState) -> RedisResult<()> {
    debug!(
        "store_local_state: Attempting to store key: {}",
        state.key().to_string()
    );
    let mut con = connect_local()?;
    con.set(state.key().to_string(), state.value())
}
pub async fn store_state_by_url(state: &SkylarkState, url: String) -> RedisResult<()> {
    debug!(
        "store_state_by_url: Attempting to store key {} at redis url {}",
        state.key().to_string(),
        url
    );
    let client = Client::open(url.clone())?;
    let mut con = client.get_multiplexed_async_connection().await?;
    con.set(state.key().to_string(), state.value()).await
}
pub async fn get_state_by_url(key: &SkylarkKey, url: &str) -> RedisResult<String> {
    debug!(
        "get_state_by_url: Attempting to receive key {} from url {}",
        key.to_string(),
        url
    );
    let client = Client::open(url)?;
    let mut con = client.get_multiplexed_async_connection().await?;
    con.get(key.to_string()).await
}
pub async fn del_local_state(key: &SkylarkKey) -> RedisResult<()> {
    debug!(
        "del_local_state: Attempting to delete key: {}",
        key.to_string()
    );
    let mut con = connect_local()?;
    con.del(key.to_string())
}
pub async fn get_global_state(key: &SkylarkKey) -> RedisResult<String> {
    debug!(
        "get_global_state: Attempting to receive key: {}",
        key.to_string()
    );
    let mut con = connect_global()?;
    con.get(key.to_string())
}

pub async fn store_global_state(state: &SkylarkState) -> RedisResult<()> {
    debug!(
        "store_global_state: Attempting to store key: {}",
        state.key().to_string()
    );
    let mut con = connect_global()?;
    con.set(state.key().to_string(), state.value())
}
pub async fn del_global_state(key: &SkylarkKey) -> RedisResult<()> {
    debug!(
        "del_global_state: Attempting to delete key: {}",
        key.to_string()
    );
    let mut con = connect_global()?;
    con.del(key.to_string())
}
