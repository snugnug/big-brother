use reqwest::Client;
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Serialize, Deserialize, Debug)]
pub struct PrInfo {
    pub id: u64,
    pub state: String,
    pub merge_commit_sha: String,
}

pub async fn get_pr_info(pr: u32) -> Result<PrInfo, Box<dyn std::error::Error>> {
    let client = Client::builder()
        .user_agent(format!("big-brother {}", env!("CARGO_PKG_VERSION")))
        .build()?;
    
    let response: PrInfo = client.get(format!("https://api.github.com/repos/nixos/nixpkgs/pulls/{}", pr))
        .send()
        .await?
	.json::<PrInfo>()
	.await?;

    // println!("{response:?}");
    Ok(response)
}
