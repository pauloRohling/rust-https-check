use anyhow::{Context, Result};
use std::time::Instant;

#[tokio::main]
async fn main() -> Result<()> {
    let urls: Vec<String> = std::env::args().skip(1).collect();

    if urls.is_empty() {
        eprintln!("Usage: rust-https-check <url1> <url2> ...");
        eprintln!("Example: rust-https-check https://example.com https://google.com");
        std::process::exit(1);
    }

    for url in urls {
        println!("\nTesting: {url}");
        let start = Instant::now();

        match fetch_url(&url).await {
            Ok(size) => {
                let elapsed = start.elapsed();
                println!("✅ SUCCESS: Received {size} bytes in {:.2?}", elapsed);
            }
            Err(err) => {
                println!("❌ FAILURE: {err:#}");
            }
        }
    }

    Ok(())
}

async fn fetch_url(url: &str) -> Result<usize> {
    let client = reqwest::Client::builder()
        .https_only(true)
        .build()
        .context("Failed to build HTTP client")?;

    let resp = client
        .get(url)
        .send()
        .await
        .with_context(|| format!("Request to {url} failed"))?;

    let status = resp.status();
    if !status.is_success() {
        anyhow::bail!("Non-OK HTTP status: {}", status);
    }

    let bytes = resp.bytes().await.context("Failed to read response body")?;
    Ok(bytes.len())
}