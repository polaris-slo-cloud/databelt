use crate::model::{SkylarkKey, SkylarkState};
use lazy_static::lazy_static;
use redis::{AsyncCommands, Client, RedisResult, Commands};
use std::sync::Mutex;
static LOCAL_REDIS_URL: &str = "redis://redis.default.svc.cluster.local:6379";
static GLOBAL_REDIS_URL: &str = "redis://redis.default.svc.cluster.local:6379";

lazy_static! {
    static ref LOCAL_REDIS_CLIENT: Mutex<Client> = {
        let client = Client::open(LOCAL_REDIS_URL).unwrap();
        Mutex::new(client)
    };
    static ref GLOBAL_REDIS_CLIENT: Mutex<Client> = {
        let client = Client::open(GLOBAL_REDIS_URL).unwrap();
        Mutex::new(client)
    };
}

pub fn connect_global() -> redis::Connection {
    GLOBAL_REDIS_CLIENT
        .lock()
        .unwrap()
        .get_connection()
        .unwrap()
}
pub fn connect_local() -> redis::Connection {
    LOCAL_REDIS_CLIENT.lock().unwrap().get_connection().unwrap()
}

pub async fn get_local_state(key: &SkylarkKey) -> RedisResult<String> {
    let mut con = connect_local();
    info!(
        "get_local_state: Attempting to receive key: {}",
        key.to_string()
    );
    con.get(key.to_string())
}

pub async fn store_local_state(state: &SkylarkState) -> RedisResult<()> {
    let mut con = connect_local();
    info!(
        "store_local_state: Attempting to store key: {}",
        state.key().to_string()
    );
    con.set(state.key().to_string(), state.value())
}
pub async fn store_state_by_url(state: &SkylarkState, url: String) -> RedisResult<()> {
    let client = Client::open(url.clone())?;
    let mut con = client.get_multiplexed_async_connection().await?;
    info!(
        "store_state_by_url: Attempting to store key {} at redis url {}",
        state.key().to_string(), url
    );
    con.set(state.key().to_string(), state.value()).await
}
pub async fn del_local_state(key: &SkylarkKey) -> RedisResult<()> {
    let mut con = connect_global();
    info!(
        "del_local_state: Attempting to delete key: {}",
        key.to_string()
    );
    con.del(key.to_string())
}
pub async fn get_global_state(key: &SkylarkKey) -> RedisResult<String> {
    let mut con = connect_global();
    info!(
        "get_global_state: Attempting to receive key: {}",
        key.to_string()
    );
    con.get(key.to_string())
}

pub async fn store_global_state(state: &SkylarkState) -> RedisResult<()> {
    let mut con = connect_global();
    info!(
        "store_global_state: Attempting to store key: {}",
        state.key().to_string()
    );
    con.set(state.key().to_string(), state.value())
}
pub async fn del_global_state(key: &SkylarkKey) -> RedisResult<()> {
    let mut con = connect_global();
    info!(
        "del_global_state: Attempting to delete key: {}",
        key.to_string()
    );
    con.del(key.to_string())
}
