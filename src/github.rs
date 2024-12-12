use reqwest::{Client, Response};
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Serialize, Deserialize, Debug)]
pub struct PrInfo {
    pub id: u64,
    pub state: String,
    pub merge_commit_sha: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PrCompare {
    pub status: String,
}

// TODO(sako):: Better error handling

pub async fn get_pr_info(client: reqwest::Client, pr: u32) -> Result<PrInfo, Box<dyn std::error::Error>> {
    let response: PrInfo = client.get(format!("https://api.github.com/repos/nixos/nixpkgs/pulls/{}", pr))
        .send()
        .await?
	.json::<PrInfo>()
	.await?;

    // println!("{response:?}");
    Ok(response)
}

// TODO(sako):: Make this optional and require an API Token to avoid ratelimits and make one that uses
// locally installed git instead to check if the commit is in a nixpkgs branch
pub async fn compare_branches_api(client: reqwest::Client, branch: &str, commit_hash: String) -> Result<bool, Box<dyn::std::error::Error>> {
    println!("asjkdfsjakdfjkwef");

    println!("{}", branch.to_string());
    println!("{}", commit_hash);
	

    let response: PrCompare = client.get(format!("https://api.github.com/repos/nixos/nixpkgs/compare/{}...{}", branch.to_string(), commit_hash))
        .send()
        .await?
	.json::<PrCompare>()
	.await?;

    println!("{:?}", response);

    // ["behind", "identical"].contains(response.status) instead? (do this later just check if it works first)
    if response.status == "behind" || response.status == "identical" {
	println!("In nixpkgs!");
	Ok(true)
    } else {
	println!("lol no");
	Ok(false)
    }
}

