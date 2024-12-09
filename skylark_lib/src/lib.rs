#![crate_type = "lib"]
#![crate_name = "skylark_lib"]
mod error;
#[allow(dead_code)]
mod model;
mod service;

pub use crate::model::{SkylarkKey, SkylarkMode, SkylarkState};
use crate::service::{
    delete_skylark_state, get_local_state, get_skylark_state, store_skylark_state,
};
use lazy_static::lazy_static;
use std::env;
use std::sync::OnceLock;
use tokio::sync::Mutex;
extern crate pretty_env_logger;
use crate::error::ParseSkylarkKeyError;
use uuid::Uuid;

#[macro_use]
extern crate log;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

lazy_static! {
    static ref PREV_SKYLARK_STATE: Mutex<SkylarkState> = Mutex::new(SkylarkState::default());
    static ref SKYLARK_KEY: Mutex<SkylarkKey> = Mutex::new(SkylarkKey::default());
    static ref SKYLARK_MODE: Mutex<SkylarkMode> = Mutex::new(SkylarkMode::Sat);
}

static SKYLARK_API_URL: OnceLock<String> = OnceLock::new();
static LOCAL_NODE_HOST: OnceLock<String> = OnceLock::new();
static LOCAL_REDIS_URL: OnceLock<String> = OnceLock::new();
static IS_INITIALIZED: OnceLock<bool> = OnceLock::new();

pub fn skylark_lib_version() -> &'static str {
    env!("CARGO_PKG_VERSION")
}

pub async fn get_state(fn_name: String, key: String, mode: SkylarkMode) -> Result<String> {
    info!("get_state");
    if IS_INITIALIZED.get().is_none() {
        debug!("get_state: Lib not yet initialized");
        match init_lib() {
            Ok(_) => {
                debug!("get_state: Lib is now initialized");
                IS_INITIALIZED.get_or_init(|| true);
            }
            Err(e) => {
                error!("Error while initializing skylark lib: {}", e.to_string());
                return Err(e.into());
            }
        }
    }
    let mut prev_state = PREV_SKYLARK_STATE.lock().await;
    let prev_key = match SkylarkKey::try_from(key.clone()) {
        Ok(key) => key,
        Err(_) => {
            error!("skylark_init: SkylarkKey conversion failed: {}", key);
            return Err(ParseSkylarkKeyError.into());
        }
    };
    prev_state.set_key(prev_key.clone());
    let mut current_key = SKYLARK_KEY.lock().await;
    current_key.set_chain_id(prev_state.key().chain_id().to_string());
    current_key.set_fn_name(fn_name);

    match get_local_state(prev_state.key()).await {
        Ok(local_state) => {
            info!("get_state: predecessor state retrieved from local KV store");
            prev_state.set_value(local_state.clone());
            return Ok(local_state);
        }
        Err(e) => {
            warn!("get_state: error while retrieving local state: {}", e);
        }
    }

    warn!("get_state: failed to fetch state from local KV store, trying API");
    match get_skylark_state(prev_state.key(), mode).await {
        Ok(cloud_state) => {
            info!("get_state: predecessor state retrieved from API");
            prev_state.set_value(cloud_state.value().clone());
            Ok(cloud_state.to_string())
        }
        Err(e) => {
            error!(
                "get_state: failed to get predecessor state from skylark api: {:?}",
                e
            );
            Err(e.into())
        }
    }
}

pub async fn store_state(
    final_state: String,
    fn_name: String,
    mode: SkylarkMode,
) -> Result<String> {
    info!("store_state");
    debug!("store_state: {}", final_state);
    if IS_INITIALIZED.get().is_none() {
        debug!("store_state: Lib not yet initialized");
        match init_lib() {
            Ok(_) => {
                debug!("store_state: Lib is now initialized");
                IS_INITIALIZED.get_or_init(|| true);
            }
            Err(e) => {
                error!("store_state: Error while initializing skylark lib: {}", e.to_string());
                return Err(e.into());
            }
        }
    }
    let mut current_key = SKYLARK_KEY.lock().await;
    if current_key.to_owned() == SkylarkKey::default() {
        current_key.set_fn_name(fn_name);
        current_key.set_chain_id(Uuid::new_v4().to_string());
    }

    let skylark_state = SkylarkState::new(current_key.clone(), final_state);
    let prev_state = PREV_SKYLARK_STATE.lock().await;
    match store_skylark_state(&skylark_state, &mode).await {
        Err(e) => {
            error!("store_state: failed to store state to skylark api: {:?}", e);
            error!("store_state: status: {:?}", e.status());
            error!("store_state: Text: {:?}", e.to_string());
            Err(e.into())
        }
        Ok(res) => {
            if prev_state.key().valid() {
                match delete_skylark_state(prev_state.key(), &mode).await {
                    Err(e) => {
                        warn!("store_state: failed to delete previous state: {:?}", e);
                    }
                    Ok(res) => {
                        info!("store_state: delete result: {}", res);
                    }
                }
            }
            info!("store_state: store state result: {}", res);
            Ok(skylark_state.key().to_string())
        }
    }
}

fn init_lib() -> Result<()> {
    info!("skylark_init: Initializing Skylark Lib");
    LOCAL_NODE_HOST.set(env::var("LOCAL_NODE_HOST")?)?;
    SKYLARK_API_URL.set(format!(
        "http://{}:{}",
        LOCAL_NODE_HOST.get().unwrap(),
        env::var("SKYLARK_API_PORT")?
    ))?;
    LOCAL_REDIS_URL.set(format!("redis://{}:6379", LOCAL_NODE_HOST.get().unwrap()))?;
    Ok(())
}
