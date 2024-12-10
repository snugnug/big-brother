use reqwest::Client;
use serde_json::Value;

pub async fn get_pr_info(pr: u32) -> Result<Value, Box<dyn std::error::Error>> {
    let client = Client::builder()
        .user_agent(format!("big-brother {}", env!("CARGO_PKG_VERSION")))
        .build()?;
    
    let response: Value = client.get(format!("https://api.github.com/repos/nixos/nixpkgs/pulls/{}", pr))
        .send()
        .await?
	.json::<Value>()
	.await?;

    // println!("{response:?}");
    Ok(response)
}
