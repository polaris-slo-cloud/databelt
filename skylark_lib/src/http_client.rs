use crate::model::{SkylarkPolicy};
use std::env;

type Result<T> = std::result::Result<T, reqwest::Error>;

pub async fn get_storage_node(
    size: usize,
    time: u128,
    policy: &SkylarkPolicy,
    destination: &String,
) -> Result<String> {
    let url = format!(
        "http://{}:{}/storage-node?size={}&time={}&policy={}&destination={}",
        env::var("LOCAL_NODE_HOST").expect("LOCAL_NODE_HOST not provided (env)"),
        env::var("SKYLARK_API_PORT").expect("SKYLARK_API_PORT not provided (env)"),
        size,
        time,
        policy,
        destination
    );
    debug!("get_skylark_state: url: {}", url);
    reqwest::get(url).await?.text().await
}

pub async fn get_neighbors() -> Result<Vec<String>> {
    let url = format!(
        "http://{}:{}/neighbors",
        env::var("LOCAL_NODE_HOST").expect("LOCAL_NODE_HOST not provided (env)"),
        env::var("SKYLARK_API_PORT").expect("SKYLARK_API_PORT not provided (env)"),
    );
    debug!("get_skylark_state: url: {}", url);
    reqwest::get(url).await?.json::<Vec<String>>().await
}
