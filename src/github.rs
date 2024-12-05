use reqwest::Client;
use std::collections::hash_map::HashMap;

pub async fn get_pr_info(pr: u32) -> Result<(), Box<dyn std::error::Error>> {
    let client = reqwest::Client::builder()
        .user_agent(format!("big-brother {}", env!("CARGO_PKG_VERSION")))
        .build()?;
    
    let response = client.get(format!("https://api.github.com/repos/nixos/nixpkgs/pulls/{}", pr))
        .send()
        .await?
	.json::<serde_json::Value>()
	.await?;

    // println!("{response:?}");
    println!("{}", serde_json::to_string_pretty(&response).unwrap());
    Ok(())
}
