#![crate_type = "lib"]
#![crate_name = "skylark_lib"]
#[allow(dead_code)]
mod model;
mod service;

use crate::model::{SkylarkKey, SkylarkMode, SkylarkState};
use crate::service::{
    delete_skylark_state, get_local_state, get_skylark_state, store_skylark_state,
};
use lazy_static::lazy_static;
use std::env;
use std::sync::OnceLock;
use tokio::sync::Mutex;
extern crate pretty_env_logger;
use uuid::Uuid;

#[macro_use]
extern crate log;

type Result<T> = std::result::Result<T, reqwest::Error>;

lazy_static! {
    static ref PREV_SKYLARK_STATE: Mutex<SkylarkState> = Mutex::new(SkylarkState::new(
        SkylarkKey::from("unknown:unknown".to_string()),
        "unknown".to_string()
    ));
}

static SKYLARK_API_PORT: OnceLock<String> = OnceLock::new();
static LOCAL_NODE_HOST: OnceLock<String> = OnceLock::new();
static SKYLARK_KEY: OnceLock<SkylarkKey> = OnceLock::new();
static SKYLARK_MODE: OnceLock<SkylarkMode> = OnceLock::new();
pub fn skylark_lib_version() -> &'static str {
    env!("CARGO_PKG_VERSION")
}

async fn get_predecessor_state(key: SkylarkKey) -> Result<String> {
    info!("skylark_lib::get_predecessor_state");

    let mut prev_state = PREV_SKYLARK_STATE.lock().await;
    prev_state.set_key(key);

    match get_local_state(&prev_state.key()).await {
        Ok(local_state) => Ok(local_state.clone()),
        Err(e) => {
            warn!("skylark_lib::get_predecessor_state: failed to get predecessor state from local KV: {:?}", e);
            info!(
                "skylark_lib::get_predecessor_state: try fetching state via skylark API: {:?}",
                e
            );
            match get_skylark_state(&prev_state.key()).await {
                Err(e) => {
                    error!("skylark_lib::get_predecessor_state: failed to get predecessor state from skylark api: {:?}", e);
                    Err(e)
                }
                Ok(state) => {
                    info!("skylark_lib::get_predecessor_state: predecessor state retrieved");
                    prev_state.set_value(state.value().clone());
                    Ok(state.value().clone())
                }
            }
        }
    }
}

pub async fn store_state(final_state: String) -> Result<String> {
    info!("skylark_lib::store_state");
    trace!("skylark_lib::store_state: {}", final_state);
    let skylark_state = SkylarkState::new(SKYLARK_KEY.get().unwrap().clone(), final_state);
    let prev_state = PREV_SKYLARK_STATE.lock().await;
    match store_skylark_state(&skylark_state, SKYLARK_MODE.get().unwrap()).await {
        Err(e) => {
            error!(
                "skylark_lib::store_state: failed to store state to skylark api: {:?}",
                e
            );
            Err(e)
        }
        Ok(..) => {
            if prev_state.key().valid() {
                match delete_skylark_state(prev_state.key()).await {
                    Err(e) => {
                        warn!(
                            "skylark_lib::store_state: failed to delete previous state: {:?}",
                            e
                        );
                    }
                    Ok(_) => {}
                }
            }
            info!("skylark_lib::store_state: Successfully stored state");
            Ok(skylark_state.key().to_string())
        }
    }
}
fn init_env(mode: &str) {
    info!("skylark_lib::init_skylark_lib::init_env: Initializing environment");

    match env::var("SKYLARK_API_PORT").unwrap() {
        Ok(port) => {
            debug!("SKYLARK_API_PORT set");
            SKYLARK_API_PORT.set(port).unwrap();
        }
        Err(_) => {
            error!("Environment variable SKYLARK_API_PORT not provided!");
            SKYLARK_API_PORT.set("30163".to_string()).unwrap();
        }
    }
    match env::var("LOCAL_NODE_HOST").unwrap() {
        Ok(host) => {
            debug!("LOCAL_NODE_HOST set");
            SKYLARK_API_PORT.set(host).unwrap();
        }
        Err(_) => {
            error!("Environment variable LOCAL_NODE_HOST not provided!");
            SKYLARK_API_PORT.set("10.0.0.34".to_string()).unwrap();
        }
    }
    match SkylarkMode::from(mode.to_string()) {
        Ok(mode) => {
            debug!("SKYLARK_MODE set");
            SKYLARK_MODE.set(mode).unwrap();
        }
        Err(_) => {
            warn!("Environment variable SKYLARK_MODE not provided! Defaulting to Satellite...");
            SKYLARK_MODE.set(SkylarkMode::Sat).unwrap();
        }
    }
}
pub async fn init_skylark_and_fetch_state(
    function_name: String,
    key: String,
    mode: &str,
) -> Result<String> {
    info!("skylark_lib::init_and_get_predecessor_state: Initializing new Skylark state");
    init_env(mode);
    let pre_key = SkylarkKey::from(key);
    SKYLARK_KEY
        .set(SkylarkKey::new(
            pre_key.chain_id().to_string(),
            function_name,
        ))
        .expect("Error while initializing Skylark API key");
    get_predecessor_state(pre_key).await
}

pub fn init_skylark(function_name: String, mode: &str) {
    info!("skylark_lib::init: Initializing Skylark Lib");
    init_env(mode);
    info!("skylark_lib::init: Initializing new Skylark state");
    SKYLARK_KEY
        .set(SkylarkKey::new(Uuid::new_v4().to_string(), function_name))
        .expect("Error while initializing Skylark API key");
}
