use anyhow::{Context, Result};

#[tokio::main]
async fn main() -> Result<()> {
    println!("Rust HTTPS Check");
    
    let url = std::env::args().nth(1)
        .or_else(|| std::env::var("URL").ok())
        .unwrap_or_else(|| "https://example.com".to_string());

    println!("-> Requesting [GET]: {url}");

    // reqwest client using rustls + *native roots*
    // /etc/ssl/certs will be required on container
    let client = reqwest::Client::builder()
        .https_only(true)
        .build()
        .context("failed to build a http client")?;

    let resp = client
        .get(&url)
        .send()
        .await
        .with_context(|| format!("request failed: {url}"))?;

    println!("Status: {}", resp.status());
    let bytes = resp.bytes().await.context("failed to read body")?;
    println!("Body: {} bytes", bytes.len());

    Ok(())
}