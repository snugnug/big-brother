use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct PrInfo {
    pub title: String,
    pub id: u64,
    pub state: String,
    pub merged: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub merge_commit_sha: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PrCompare {
    pub status: String,
}

// TODO(sako):: Better error handling

pub async fn get_pr_info(
    client: reqwest::Client,
    pr: u64,
) -> Result<PrInfo, Box<dyn std::error::Error>> {
    let api_key = std::env::var("GITHUB_API_KEY").ok();

    let mut request = client.get(format!(
        "https://api.github.com/repos/nixos/nixpkgs/pulls/{}",
        pr
    ));

    if api_key != None {
        request = request.header(
            reqwest::header::AUTHORIZATION,
            format!("Bearer {}", api_key.unwrap()),
        );
    };

    let info = request.send().await?;

    tracing::debug!("{}", info.status());

    if info.status().is_success() {
        let pr_info = info.json::<PrInfo>().await?;
        tracing::debug!("{}", serde_json::to_string_pretty(&pr_info).unwrap());
        Ok(pr_info)
    } else {
        Err(format!("failed with error code {}", info.status()).into())
    }
}

// TODO(sako):: Make this optional and require an API Token to avoid ratelimits and make one that uses
// locally installed git instead to check if the commit is in a nixpkgs branch
pub async fn compare_branches_api(
    client: reqwest::Client,
    branch: String,
    commit_hash: String,
) -> Result<bool, Box<dyn ::std::error::Error + Send + Sync>> {
    tracing::debug!("{}", branch.to_string());
    tracing::debug!("{}", commit_hash);

    let api_key = std::env::var("GITHUB_API_KEY").ok();

    let mut request = client.get(format!(
        "https://api.github.com/repos/nixos/nixpkgs/compare/{}...{}",
        branch.to_string(),
        commit_hash
    ));

    if api_key != None {
        request = request.header(
            reqwest::header::AUTHORIZATION,
            format!("Bearer {}", api_key.unwrap()),
        );
    };

    let response = request.send().await?;

    if response.status().is_success() {
        let output = response.json::<PrCompare>().await.unwrap();
        if output.status == "behind" || output.status == "identical" {
            tracing::debug!("In nixpkgs!");
            return Ok(true);
        } else {
            tracing::debug!("lol no");
            return Ok(false);
        }
    } else {
        return Err(format!("failed with error code {}", response.status()).into());
    }
}
