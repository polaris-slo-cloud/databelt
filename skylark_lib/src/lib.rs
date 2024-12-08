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
    static ref PREV_SKYLARK_STATE: Mutex<SkylarkState> = Mutex::new(SkylarkState::new(
        SkylarkKey::try_from("unknown:unknown".to_string()).unwrap(),
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
    info!("get_predecessor_state");

    let mut prev_state = PREV_SKYLARK_STATE.lock().await;
    prev_state.set_key(key);

    match get_local_state(&prev_state.key()).await {
        Ok(local_state) => Ok(local_state.clone()),
        Err(e) => {
            warn!(
                "get_predecessor_state: failed to get predecessor state from local KV: {:?}",
                e
            );
            info!(
                "get_predecessor_state: try fetching state via skylark API: {:?}",
                e
            );
            match get_skylark_state(&prev_state.key()).await {
                Err(e) => {
                    error!("get_predecessor_state: failed to get predecessor state from skylark api: {:?}", e);
                    Err(e.into())
                }
                Ok(state) => {
                    info!("get_predecessor_state: predecessor state retrieved");
                    prev_state.set_value(state.value().clone());
                    Ok(state.value().clone())
                }
            }
        }
    }
}

pub async fn store_state(final_state: String) -> Result<String> {
    info!("store_state");
    trace!("store_state: {}", final_state);
    let skylark_state = SkylarkState::new(SKYLARK_KEY.get().unwrap().clone(), final_state);
    let prev_state = PREV_SKYLARK_STATE.lock().await;
    match store_skylark_state(&skylark_state, SKYLARK_MODE.get().unwrap()).await {
        Err(e) => {
            error!("store_state: failed to store state to skylark api: {:?}", e);
            error!("store_state: status: {:?}", e.status());
            error!("store_state: Text: {:?}", e.to_string());
            Err(e.into())
        }
        Ok(res) => {
            if prev_state.key().valid() {
                match delete_skylark_state(prev_state.key()).await {
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
fn init_env() -> Result<()> {
    info!("skylark_init::init_env: Initializing environment");
    SKYLARK_API_PORT.set(env::var("SKYLARK_API_PORT")?)?;
    LOCAL_NODE_HOST.set(env::var("LOCAL_NODE_HOST")?)?;
    Ok(())
}
pub async fn skylark_init(
    fn_name: String,
    key: Option<String>,
    mode: SkylarkMode,
) -> Result<String> {
    info!("skylark_init: Initializing new Skylark state");
    match init_env() {
        Ok(_) => {}
        Err(e) => {
            error!("Error while initializing skylark lib: {}", e.to_string());
            return Err(e.into());
        }
    }
    SKYLARK_MODE
        .set(mode)
        .expect("skylark_init: Failed initializing mode");
    match key {
        Some(key) => {
            info!("skylark_init: init_key: {}", &key);
            let pre_key = match SkylarkKey::try_from(key.clone()) {
                Ok(key) => key,
                Err(_) => {
                    error!("skylark_init: SkylarkKey conversion failed: {}", key);
                    return Err(ParseSkylarkKeyError.into());
                }
            };
            match get_predecessor_state(pre_key.clone()).await {
                Ok(state) => {
                    SKYLARK_KEY
                        .set(SkylarkKey::new(pre_key.chain_id().to_string(), fn_name))
                        .expect("skylark_init: SkylarkKey already initialized");
                    Ok(state)
                }
                Err(e) => {
                    error!(
                        "skylark_init: get_predecessor_state failed: {:?}",
                        e.to_string()
                    );
                    Err(e)
                }
            }
        }
        None => {
            info!("skylark_init: no predecessor key given, initializing new key");
            SKYLARK_KEY
                .set(SkylarkKey::new(Uuid::new_v4().to_string(), fn_name))
                .expect("skylark_init: SkylarkKey already initialized");
            Ok(String::new())
        }
    }
}
