#![deny(warnings)]

// This is using the `tokio` runtime. You'll need the following dependency:
//
// `tokio = { version = "1", features = ["full"] }`
#[cfg(any(not(target_arch = "wasm32"), target_os = "wasi"))]
#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), reqwest::Error> {
    // Some simple CLI args requirements...
    let url = match std::env::args().nth(1) {
        Some(url) => url,
        None => {
            println!("No CLI URL provided, using default.");
            "http://eu.httpbin.org/get?msg=WasmEdge".into()
        }
    };

    eprintln!("Fetching {:?}...", url);
    let res = reqwest::get(url).await?;
    eprintln!("Response: {:?} {}", res.version(), res.status());
    eprintln!("Headers: {:#?}\n", res.headers());
    let body = res.text().await?;
    println!("{}", body);

    let url = "http://10.152.183.152/neighbors";

    eprintln!("Fetching {:?}...", url);
    let mut headers = reqwest::header::HeaderMap::new();
    headers.insert(reqwest::header::HOST, reqwest::header::HeaderValue::from_static("skylark-neighbors.default.svc.cluster.local"));
    let client = reqwest::Client::builder().default_headers(headers).build()?;
    let res = client.get(url).send().await?;
    eprintln!("Response: {:?} {}", res.version(), res.status());
    eprintln!("Headers: {:#?}\n", res.headers());
    let body = res.text().await?;
    println!("{}", body);

    Ok(())
}

// The [cfg(not(target_arch = "wasm32"))] above prevent building the tokio::main function
// for wasm32 target, because tokio isn't compatible with wasm32.
// If you aren't building for wasm32, you don't need that line.
// The two lines below avoid the "'main' function not found" error when building for wasm32 target.
#[cfg(all(target_arch = "wasm32", not(target_os = "wasi")))]
fn main() {}
