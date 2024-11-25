#![crate_type = "lib"]
#![crate_name = "skylark_lib"]
mod model;
mod service;

use crate::model::{SkylarkKey, SkylarkState};
use crate::service::{delete_skylark_state, get_skylark_state, store_skylark_state};
use lazy_static::lazy_static;
use std::env;
use std::ops::Deref;
use std::sync::{Mutex, OnceLock};
extern crate pretty_env_logger;
use uuid::Uuid;

#[macro_use]
extern crate log;

type Result<T> = std::result::Result<T, reqwest::Error>;

lazy_static! {
    static ref PREV_SKYLARK_STATE: Mutex<SkylarkState> = Mutex::new(SkylarkState::new(
        SkylarkKey::from("".to_string()),
        "".to_string()
    ));
    static ref SKYLARK_STATE: Mutex<SkylarkState> = Mutex::new(SkylarkState::new(
        SkylarkKey::from("".to_string()),
        "".to_string()
    ));
}
static SKYLARK_API_URL: OnceLock<String> = OnceLock::new();
pub fn get_version() -> &'static str {
    env!("CARGO_PKG_VERSION")
}

pub async fn get_predecessor_state(fn_name: String, key: Option<String>) -> Result<String> {
    info!("skylark::get_predecessor_state");
    let mut new_state = SKYLARK_STATE.lock().unwrap();
    match key {
        Some(k) => {
            info!("skylark::get_predecessor_state: key from predecessor state given, trying to fetch predecessor state");
            let mut prev_state = PREV_SKYLARK_STATE.lock().unwrap();
            prev_state.set_key(SkylarkKey::from(k));
            new_state.set_key(SkylarkKey::new(
                prev_state.key().chain_id().to_string(),
                fn_name.clone(),
            ));
            match get_skylark_state(&prev_state.key()).await {
                Err(e) => {
                    error!("skylark::get_predecessor_state: failed to get predecessor state from skylark api: {:?}", e);
                    Err(e)
                }
                Ok(state) => {
                    info!("skylark::get_predecessor_state: predecessor state retrieved");
                    prev_state.set_value(state.value().clone());
                    Ok(state.value().clone())
                }
            }
        }
        None => {
            info!("skylark::get_predecessor_state: No predecessor key given, initializing new chain id and state");
            new_state.set_key(SkylarkKey::new(Uuid::new_v4().to_string(), fn_name));
            Ok(new_state.value().clone())
        }
    }
}
pub async fn store_state(final_state: String) -> Result<()> {
    info!("skylark::store_state");
    trace!("skylark::store_state: {}", final_state);
    let mut skylark_state = SKYLARK_STATE.lock().unwrap();
    let prev_state = PREV_SKYLARK_STATE.lock().unwrap();
    skylark_state.set_value(final_state);
    match store_skylark_state(skylark_state.deref()).await {
        Err(e) => {
            error!(
                "skylark::store_state: failed to store state to skylark api: {:?}",
                e
            );
            Err(e)
        }
        Ok(..) => {
            if prev_state.key().valid() {
                match delete_skylark_state(prev_state.key()).await {
                    Err(e) => {
                        warn!("skylark::store_state: failed to delete previous state: {:?}",e);
                    }
                    Ok(_) => {}
                }
            }
            info!("skylark::store_state: Successfully stored state");
            Ok(())
        }
    }
}

fn init() {
    info!("skylark::init: Initializing Skylark Lib");
    pretty_env_logger::init_timed();
    SKYLARK_API_URL
        .set(
            env::var("SKYLARK_API_URL")
                .unwrap_or("http://skylark-api.default.svc.cluster.local".to_string()),
        )
        .expect("TODO: panic message");
}
