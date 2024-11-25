use crate::model::{SkylarkKey, SkylarkState};
use crate::SKYLARK_API_URL;

type Result<T> = std::result::Result<T, reqwest::Error>;

pub async fn get_skylark_state(key: &SkylarkKey) -> Result<SkylarkState> {
    let url = format!("{}/{}", SKYLARK_API_URL.get().unwrap(), key.to_string());
    info!("skylark::get_skylark_state: url: {}", url);
    reqwest::get(url).await?.json::<SkylarkState>().await
}

pub async fn store_skylark_state(state: &SkylarkState) -> Result<SkylarkState> {
    info!(
        "skylark::store_skylark_state: state: {}",
        state.value().clone()
    );
    reqwest::Client::new()
        .post(SKYLARK_API_URL.get().unwrap())
        .json::<SkylarkState>(state)
        .send()
        .await?
        .json::<SkylarkState>()
        .await
}

pub async fn delete_skylark_state(key: &SkylarkKey) -> Result<()> {
    let url = format!("{}/{}", SKYLARK_API_URL.get().unwrap(), key.to_string());
    info!("skylark::get_skylark_state: url: {}", url);
    reqwest::Client::new()
        .delete(SKYLARK_API_URL.get().unwrap())
        .send()
        .await?
        .json::<()>()
        .await
}
