mod parsing_structs;

use anyhow::{Context, Result};

#[tokio::main]
async fn main() -> Result<()> {
    let response = reqwest::get("https://api.getcat.dev/")
        .await
        .context("Failed to get response")?;

    let json = response
        .json::<parsing_structs::root::Root>()
        .await
        .context("Failed to parse json")?;

    println!("Successful response, {}", json.url);

    Ok(())
}
