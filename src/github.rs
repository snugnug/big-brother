use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::collections::hash_map::HashMap;

#[derive(Deserialize, Serialize, Debug)]
pub struct PrInfo {
    // Holy shit there are so many prs on nixpkgs
    number: u64,
    state: String,
    title: String,
}

pub async fn get_pr_info(pr: u32) -> Result<serde_json::value::Value, Box<dyn std::error::Error>> {
    let client = reqwest::Client::builder()
        .user_agent(format!("big-brother {}", env!("CARGO_PKG_VERSION")))
        .build()?;
    
    let response: serde_json::value::Value = client.get(format!("https://api.github.com/repos/nixos/nixpkgs/pulls/{}", pr))
        .send()
        .await?
	.json::<serde_json::value::Value>()
	.await?;

    // println!("{response:?}");
    Ok(response)
}
