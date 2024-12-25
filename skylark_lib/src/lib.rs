#![crate_type = "lib"]
#![crate_name = "skylark_lib"]
mod error;
mod http_client;
#[allow(dead_code)]
mod model;
mod redis_client;

pub use crate::model::{
    SkylarkBundledState, SkylarkKey, SkylarkPolicy, SkylarkState, SkylarkStorageType,
};
use lazy_static::lazy_static;
use std::env;
use std::sync::OnceLock;
use std::time::Instant;
use tokio::sync::Mutex;

extern crate pretty_env_logger;
use crate::error::{ParseSkylarkKeyError, SkylarkStateError};
use crate::http_client::{get_neighbors, get_storage_node};
use crate::redis_client::{
    del_state_by_host, get_bundled_state_by_host, get_single_state_by_host,
    set_bundled_state_by_host, set_single_state_by_host,
};
use uuid::Uuid;

#[macro_use]
extern crate log;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

lazy_static! {
    static ref SKYLARK_KEY: Mutex<SkylarkKey> = Mutex::new(SkylarkKey::default());
    static ref TIMER: Mutex<Instant> = Mutex::new(Instant::now());
}

static NEIGHBOR_HOSTS: OnceLock<Vec<String>> = OnceLock::new();

pub fn skylark_lib_version() -> &'static str {
    env!("CARGO_PKG_VERSION")
}

async fn check_neighbors() {
    if NEIGHBOR_HOSTS.get().is_none() {
        debug!("check_neighbors: neighbors not set, fetching them now");
        match get_neighbors().await {
            Ok(neighbors) => {
                debug!("check_neighbors: neighbors: {:?}", neighbors);
                NEIGHBOR_HOSTS.set(neighbors).unwrap();
            }
            Err(e) => {
                warn!("check_neighbors: failed to get neighbors: {:?}", e);
            }
        }
    }
}

pub async fn get_single_state(
    key: &String,
    policy: &SkylarkPolicy,
    storage_type: &SkylarkStorageType,
) -> Result<String> {
    // Fetch state for given `key` based on policy
    // Serverless: 1. global
    // Random & Skylark try fetch in this order: 1. local 2. neighbors 3. global
    info!("get_single_state");
    debug!("get_single_state: key {}", key);
    check_neighbors().await;
    let prev_key = match SkylarkKey::try_from(key.clone()) {
        Ok(key) => key,
        Err(_) => {
            error!("skylark_init: SkylarkKey conversion failed: {}", key);
            return Err(ParseSkylarkKeyError.into());
        }
    };
    let mut current_key = SKYLARK_KEY.lock().await;
    current_key.set_chain_id(prev_key.chain_id().to_string());
    current_key.set_task_id(Uuid::new_v4().to_string());
    info!("get_single_state: fetch state based on policy: {}", policy);
    if !SkylarkPolicy::Serverless.eq(policy) {
        // Skylark or Random Policy
        // Try fetching state from local store
        match get_single_state_by_host(&prev_key, &env::var("LOCAL_NODE_HOST")?, storage_type).await
        {
            Ok(local_state) => {
                info!("get_single_state: predecessor state retrieved from local KV store");
                debug!("get_single_state: local state size: {}", local_state.len());
                return Ok(local_state);
            }
            Err(e) => {
                info!("get_single_state: state not found in local store: {}", e);
            }
        }
        if NEIGHBOR_HOSTS.get().unwrap().len() == 0 {
            warn!("Not aware of any neighbors!");
        }
        // Try fetching state from neighbors
        for neighbor_host in NEIGHBOR_HOSTS.get().unwrap() {
            debug!("get_single_state: trying neighbor: {}", neighbor_host);
            match get_single_state_by_host(&prev_key, neighbor_host, storage_type).await {
                Ok(neighbor_state) => {
                    info!("get_single_state: predecessor state retrieved from neighbor KV store");
                    debug!("get_single_state: state size: {}", neighbor_state.len());
                    return Ok(neighbor_state);
                }
                Err(e) => {
                    info!("get_single_state: state not found in neighbor store: {}", e);
                }
            }
        }
        info!("get_single_state: state not found in either neighbor");
    }

    // Finally fetch from Global store
    match get_single_state_by_host(&prev_key, &env::var("GLOBAL_STATE_HOST")?, storage_type).await {
        Ok(global_state) => {
            info!("get_single_state: predecessor state retrieved from global KV store");
            debug!(
                "get_single_state: global state size: {}",
                global_state.len()
            );
            Ok(global_state)
        }
        Err(e) => {
            error!("get_single_state: state not found in global store: {}", e);
            Err(SkylarkStateError.into())
        }
    }
}
pub async fn get_bundled_state(
    key: &String,
    policy: &SkylarkPolicy,
) -> Result<Vec<(String, String)>> {
    // Fetch bundled state for given `key` based on policy
    // Serverless: 1. global
    // Random & Skylark try fetch in this order: 1. local 2. neighbors 3. global
    info!("get_bundled_state");
    debug!("get_bundled_state: key {}", key);
    check_neighbors().await;
    let prev_key = match SkylarkKey::try_from(key.clone()) {
        Ok(key) => key,
        Err(_) => {
            error!("skylark_init: SkylarkKey conversion failed: {}", key);
            return Err(ParseSkylarkKeyError.into());
        }
    };
    let mut current_key = SKYLARK_KEY.lock().await;
    current_key.set_chain_id(prev_key.chain_id().to_string());
    current_key.set_task_id(Uuid::new_v4().to_string());
    info!("get_bundled_state: fetch state based on policy: {}", policy);
    if !SkylarkPolicy::Serverless.eq(policy) {
        // Skylark or Random Policy
        // Try fetching state from local store
        match get_bundled_state_by_host(&prev_key, &env::var("LOCAL_NODE_HOST")?).await {
            Ok(local_state) => {
                info!("get_bundled_state: predecessor state retrieved from local KV store");
                debug!("get_bundled_state: local state size: {}", local_state.len());
                return Ok(local_state);
            }
            Err(e) => {
                info!("get_bundled_state: state not found in local store: {}", e);
            }
        }
        if NEIGHBOR_HOSTS.get().unwrap().len() == 0 {
            warn!("Not aware of any neighbors!");
        }
        // Try fetching state from neighbors
        for neighbor_host in NEIGHBOR_HOSTS.get().unwrap() {
            debug!("get_bundled_state: trying neighbor: {}", neighbor_host);
            match get_bundled_state_by_host(&prev_key, neighbor_host).await {
                Ok(neighbor_state) => {
                    info!("get_bundled_state: predecessor state retrieved from neighbor KV store");
                    debug!("get_bundled_state: state size: {}", neighbor_state.len());
                    return Ok(neighbor_state);
                }
                Err(e) => {
                    info!(
                        "get_bundled_state: state not found in neighbor store: {}",
                        e
                    );
                }
            }
        }
        info!("get_bundled_state: state not found in either neighbor");
    }

    // Finally fetch from Global store
    match get_bundled_state_by_host(&prev_key, &env::var("GLOBAL_STATE_HOST")?).await {
        Ok(global_state) => {
            info!("get_bundled_state: predecessor state retrieved from global KV store");
            debug!(
                "get_bundled_state: global state size: {}",
                global_state.len()
            );
            Ok(global_state)
        }
        Err(e) => {
            error!("get_bundled_state: state not found in global store: {}", e);
            Err(SkylarkStateError.into())
        }
    }
}
pub async fn store_single_state(
    final_state: String,
    destination_host: &String,
    policy: &SkylarkPolicy,
    storage_type: &SkylarkStorageType,
) -> Result<String> {
    // Fetch target host to store state based on `policy` for `destination` host.
    info!("store_single_state: incoming");
    debug!("store_single_state length: {}", final_state.len());
    let mut current_key = SKYLARK_KEY.lock().await;
    debug!("store_single_state: current key loaded");
    if current_key.clone().eq(&SkylarkKey::default()) {
        current_key.set_task_id(Uuid::new_v4().to_string());
        current_key.set_chain_id(Uuid::new_v4().to_string());
    }
    debug!("store_single_state: calling skylark api");
    let data_size: i16 = final_state.len() as i16;
    let skylark_state = SkylarkState::new(current_key.clone(), final_state);
    debug!("store_single_state: data_size: {:?}", data_size);
    let fn_exec_time = TIMER.lock().await.elapsed().as_millis() as i16;
    debug!("store_single_state: fn_exec_time: {:?}", fn_exec_time);

    // Elect storage host
    let elected_host =
        match get_storage_node(data_size, fn_exec_time, policy, destination_host).await {
            Ok(host) => {
                debug!("store_single_state: elected host: {}", host);
                host
            }
            Err(e) => {
                error!("store_single_state: status: {:?}", e.status());
                error!("store_single_state: Text: {:?}", e.to_string());
                return Err(e.into());
            }
        };

    // Store state to elected and global store
    match set_single_state_by_host(&skylark_state, &elected_host, storage_type).await {
        Ok(_) => {
            debug!("store_single_state: successfully stored state");
        }
        Err(e) => {
            error!("store_single_state: failed to store state: {}", e);
        }
    }
    // Check if elected host was already global state host
    if !elected_host.eq(&env::var("GLOBAL_STATE_HOST")?) {
        match set_single_state_by_host(
            &skylark_state,
            &env::var("GLOBAL_STATE_HOST")?,
            storage_type,
        )
        .await
        {
            Ok(_) => {
                debug!("store_single_state: successfully stored global state");
            }
            Err(e) => {
                error!("store_single_state: failed to store global state: {}", e);
                return Err(e.into());
            }
        }
    }
    info!("store_single_state: successfully stored new state");
    Ok(current_key.to_string())
}

pub async fn store_bundled_state(
    final_state: Vec<(String, String)>,
    destination_host: &String,
    policy: &SkylarkPolicy,
) -> Result<String> {
    // Fetch target host to store state based on `policy` for `destination` host.
    info!("store_bundled_state: incoming");
    debug!("store_bundled_state: length: {}", final_state.len());
    let mut current_key = SKYLARK_KEY.lock().await;
    debug!("store_bundled_state: current key loaded");
    if current_key.to_owned() == SkylarkKey::default() {
        current_key.set_task_id(Uuid::new_v4().to_string());
        current_key.set_chain_id(Uuid::new_v4().to_string());
    }
    debug!("store_bundled_state: calling skylark api");
    let data_size: i16 = final_state.len() as i16;
    let skylark_state = SkylarkBundledState::new(current_key.clone(), final_state);
    debug!("store_bundled_state: data_size: {:?}", data_size);
    let fn_exec_time = TIMER.lock().await.elapsed().as_millis() as i16;
    debug!("store_bundled_state: fn_exec_time: {:?}", fn_exec_time);

    // Elect storage host
    let elected_host =
        match get_storage_node(data_size, fn_exec_time, policy, destination_host).await {
            Ok(host) => {
                debug!("store_bundled_state: elected host: {}", host);
                host
            }
            Err(e) => {
                error!("store_bundled_state: status: {:?}", e.status());
                error!("store_bundled_state: Text: {:?}", e.to_string());
                return Err(e.into());
            }
        };

    // Store state to elected and global store
    match set_bundled_state_by_host(&skylark_state, &elected_host).await {
        Ok(_) => {
            debug!("store_bundled_state: successfully stored state");
        }
        Err(e) => {
            error!("store_bundled_state: failed to store state: {}", e);
        }
    }
    // Check if elected host was already global state host
    if !elected_host.eq(&env::var("GLOBAL_STATE_HOST")?) {
        match set_bundled_state_by_host(
            &skylark_state,
            &env::var("GLOBAL_STATE_HOST")?
        )
        .await
        {
            Ok(_) => {
                debug!("store_bundled_state: successfully stored global state");
            }
            Err(e) => {
                error!("store_bundled_state: failed to store global state: {}", e);
                return Err(e.into());
            }
        }
    }
    info!("store_bundled_state: successfully stored new state");
    Ok(current_key.to_string())
}

pub async fn delete_state(key: String, storage_type: &SkylarkStorageType) -> Result<()> {
    // Deletes state from previous host and global state host.
    info!("delete_state: Incoming");
    let skylark_key = SkylarkKey::try_from(key.clone()).unwrap();
    match del_state_by_host(&skylark_key, &env::var("LOCAL_NODE_HOST")?, storage_type).await {
        Ok(_) => {
            debug!("delete_state: deleted from local host: {}", key);
        }
        Err(e) => {
            warn!(
                "delete_state: failed to delete state from previous host: {:?}",
                e
            );
        }
    }
    if NEIGHBOR_HOSTS.get().unwrap().len() == 0 {
        warn!("delete_state: Not aware of any neighbors!");
    }
    // Try deleting from neighbors
    for neighbor_host in NEIGHBOR_HOSTS.get().unwrap() {
        debug!("delete_state: trying neighbor: {}", neighbor_host);
        match del_state_by_host(&skylark_key, neighbor_host, storage_type).await {
            Ok(_) => {
                debug!("delete_state: deleted from neighbor host: {}", key);
            }
            Err(e) => {
                warn!(
                    "delete_state: failed to delete state from previous host: {:?}",
                    e
                );
            }
        }
    }
    info!("delete_state: state not found in either neighbor");

    // Delete global state
    match del_state_by_host(&skylark_key, &env::var("GLOBAL_STATE_HOST")?, storage_type).await {
        Ok(_) => {
            info!("delete_state: OK deleted state from previous and global state host");
            Ok(())
        }
        Err(e) => {
            error!(
                "delete_state: failed to delete state from global host: {:?}",
                e
            );
            Err(e.into())
        }
    }
}

pub async fn start_timing() {
    TIMER.lock().await.clone_from(&Instant::now());
    debug!("TIMER started");
}
