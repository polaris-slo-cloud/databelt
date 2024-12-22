use reqwest::{Client};
use serde::de::DeserializeOwned;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

pub async fn get_from_url<T>(url: &str) -> Result<T>
where
    T: DeserializeOwned,
{
    debug!("skylark::get_from_node_provider: url: {}", url);
    let response = Client::new().get(url).send().await;
    match response {
        Ok(res) => {
            match res.status().is_success() {
                true => {
                    let json = res.json::<T>().await;
                    match json {
                        Ok(json_data) => {
                            Ok(json_data)
                        },
                        Err(e) => {
                            Err(Box::new(e))
                        },
                    }
                },
                false => {
                    Err(Box::new(std::io::Error::new(
                        std::io::ErrorKind::Other,
                        format!("Request failed with status: {}", res.status()),
                    )))
                },
            }
        },
        Err(err) => {
            Err(Box::new(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!("Request failed : {}", err.to_string()),
            )))
        },
    }
}