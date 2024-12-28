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
    static ref OUTPUT_KEY: Mutex<SkylarkKey> = Mutex::new(SkylarkKey::default());
    static ref TIMER: Mutex<Instant> = Mutex::new(Instant::now());
}

static NEIGHBOR_HOSTS: OnceLock<Vec<String>> = OnceLock::new();

pub fn skylark_lib_version() -> &'static str {
    env!("CARGO_PKG_VERSION")
}

pub async fn init_new_chain(){
    debug!("init_new_chain: Initializing new SkylarkKey");
    let mut new_key = OUTPUT_KEY.lock().await;
    new_key.set_task_id(Uuid::new_v4().to_string());
    new_key.set_chain_id(Uuid::new_v4().to_string());
    new_key.set_node_id(env::var("LOCAL_NODE_HOST").unwrap());
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
    // Stateless: 1. global
    // Random & Skylark try fetch in this order: 1. local 2. neighbors 3. global
    debug!("get_single_state: key {}", key);
    check_neighbors().await;
    let input_key = match SkylarkKey::try_from(key.clone()) {
        Ok(key) => key,
        Err(_) => {
            error!("skylark_init: SkylarkKey conversion failed: {}", key);
            return Err(ParseSkylarkKeyError.into());
        }
    };
    let mut output_key = OUTPUT_KEY.lock().await;
    output_key.set_chain_id(input_key.chain_id().to_string());
    output_key.set_node_id(env::var("LOCAL_NODE_HOST").unwrap());
    output_key.set_task_id(Uuid::new_v4().to_string());
    debug!("get_single_state: fetch state based on policy: {}", policy);
    match get_single_state_by_host(&input_key, input_key.node_id(), storage_type).await
    {
        Ok(local_state) => {
            info!("HOPS: {} -> {}", env::var("LOCAL_NODE_HOST").unwrap(), input_key.node_id());
            debug!("get_single_state: local state size: {}", local_state.len());
            Ok(local_state)
        }
        Err(e) => {
            error!("get_single_state: state not found! {}", e);
            Err(SkylarkStateError.into())
        }
    }
}
pub async fn get_bundled_state(
    key: &String,
    policy: &SkylarkPolicy,
) -> Result<Vec<(String, String)>> {
    // Fetch bundled state for given `key` based on policy
    // Stateless: 1. global
    // Random & Skylark try fetch in this order: 1. local 2. neighbors 3. global
    debug!("get_bundled_state: key {}", key);
    check_neighbors().await;
    let input_key = match SkylarkKey::try_from(key.clone()) {
        Ok(key) => key,
        Err(_) => {
            error!("skylark_init: SkylarkKey conversion failed: {}", key);
            return Err(ParseSkylarkKeyError.into());
        }
    };
    let mut output_key = OUTPUT_KEY.lock().await;
    output_key.set_chain_id(input_key.chain_id().to_string());
    output_key.set_node_id(env::var("LOCAL_NODE_HOST").unwrap());
    output_key.set_task_id(Uuid::new_v4().to_string());
    debug!("get_bundled_state: fetch state based on policy: {}", policy);
    match get_bundled_state_by_host(&input_key, input_key.node_id()).await {
        Ok(bundled_state) => {
            info!("HOPS: {} -> {}", env::var("LOCAL_NODE_HOST").unwrap(), input_key.node_id());
            debug!("get_single_state: local state size: {}", bundled_state.len());
            Ok(bundled_state)
        }
        Err(e) => {
            error!("get_single_state: bundled state not found! {}", e);
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
    debug!("store_single_state: incoming");
    debug!("store_single_state: calling skylark api");
    let data_size = final_state.len();
    debug!("store_single_state: data_size: {:?}", data_size);
    let fn_exec_time = TIMER.lock().await.elapsed().as_millis();
    debug!("store_single_state: fn_exec_time: {:?}", fn_exec_time);

    // Elect storage host
    let elected_host =
        match get_storage_node(data_size, fn_exec_time, policy, destination_host).await {
            Ok(host) => {
                info!("store_single_state: elected host: {}", host);
                host
            }
            Err(e) => {
                error!("store_single_state: status: {:?}", e.status());
                error!("store_single_state: Text: {:?}", e.to_string());
                return Err(e.into());
            }
        };
    let mut output_key = OUTPUT_KEY.lock().await;
    debug!("store_single_state: current key loaded");
    if output_key.clone().eq(&SkylarkKey::default()) {
        output_key.set_task_id(Uuid::new_v4().to_string());
        output_key.set_chain_id(Uuid::new_v4().to_string());
    }
    output_key.set_node_id(elected_host.clone());
    let skylark_state = SkylarkState::new(output_key.clone(), final_state);
    // Store state to elected and global store
    let tde = Instant::now();
    match set_single_state_by_host(&skylark_state, &elected_host, storage_type).await {
        Ok(_) => {
            info!("T(de): {:?}ms", tde.elapsed().as_millis());
            debug!("store_single_state: successfully stored state");
        }
        Err(e) => {
            error!("store_single_state: failed to store state: {}", e);
        }
    }
    Ok(output_key.to_string())
}

pub async fn store_bundled_state(
    final_state: Vec<(String, String)>,
    destination_host: &String,
    policy: &SkylarkPolicy,
) -> Result<String> {
    // Fetch target host to store state based on `policy` for `destination` host.
    debug!("store_bundled_state: incoming");
    debug!("store_bundled_state: calling skylark api");
    let data_size = final_state.len();
    debug!("store_bundled_state: data_size: {:?}", data_size);
    let fn_exec_time = TIMER.lock().await.elapsed().as_millis();
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

    let mut output_key = OUTPUT_KEY.lock().await;
    debug!("store_single_state: current key loaded");
    if output_key.clone().eq(&SkylarkKey::default()) {
        output_key.set_task_id(Uuid::new_v4().to_string());
        output_key.set_chain_id(Uuid::new_v4().to_string());
    }
    output_key.set_node_id(elected_host.clone());
    let skylark_state = SkylarkBundledState::new(output_key.clone(), final_state);
    // Store state to elected and global store
    let tde = Instant::now();
    match set_bundled_state_by_host(&skylark_state, &elected_host).await {
        Ok(_) => {
            info!("T(de): {:?}ms", tde.elapsed().as_millis());
            debug!("store_bundled_state: successfully stored state");
        }
        Err(e) => {
            error!("store_bundled_state: failed to store state: {}", e);
        }
    }
    debug!("store_bundled_state: successfully stored new state");
    Ok(output_key.to_string())
}

pub async fn delete_state(key: String, storage_type: &SkylarkStorageType) -> Result<()> {
    // Deletes state from previous host and global state host.
    debug!("delete_state: Incoming");
    let parsed_key = SkylarkKey::try_from(key.clone()).unwrap();
    match del_state_by_host(&parsed_key, parsed_key.node_id(), storage_type).await {
        Ok(_) => {
            debug!("delete_state: deleted from host: {}", parsed_key.node_id());
        }
        Err(e) => {
            warn!(
                "delete_state: failed to delete state from previous host: {:?}",
                e
            );
        }
    }
    Ok(())
}

pub async fn start_timing() {
    TIMER.lock().await.clone_from(&Instant::now());
    debug!("TIMER started");
}
