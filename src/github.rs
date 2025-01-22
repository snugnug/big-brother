use axum::http::status;
use reqwest::{Client, Request, Response};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::error::Error;
use std::fmt::{self, Display, Formatter};

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

#[derive(Debug)]
pub enum EpicFail {
    RequestError(reqwest::Error),
    ParseError(serde_json::Error),
}

impl Error for EpicFail {}

impl Display for EpicFail {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            Self::RequestError(e) => write!(f, "Request failed! {}", e),
            Self::ParseError(e) => write!(f, "Parse failed! {}", e),
        }
    }
}

impl From<reqwest::Error> for EpicFail {
    fn from(err: reqwest::Error) -> EpicFail {
        EpicFail::RequestError(err)
    }
}

// TODO(sako):: Better error handling

pub async fn get_pr_info(
    client: reqwest::Client,
    pr: u64,
) -> Result<PrInfo, Box<dyn std::error::Error>> {
    let info = client
        .get(format!(
            "https://api.github.com/repos/nixos/nixpkgs/pulls/{}",
            pr
        ))
        .send()
        .await?;
    // .json::<PrInfo>()
    // .await?;

    tracing::debug!("{}", info.status());

    if info.status().is_success() {
        let pr_info = info.json::<PrInfo>().await?;
        Ok(pr_info)
    } else {
        Err(format!("failed with error code {}", info.status()).into())
    }

    // let bleh = info.json::<PrInfo>().await?;

    // Ok(bleh)
}

// TODO(sako):: Make this optional and require an API Token to avoid ratelimits and make one that uses
// locally installed git instead to check if the commit is in a nixpkgs branch
pub async fn compare_branches_api(
    client: reqwest::Client,
    branch: &str,
    commit_hash: String,
) -> Result<bool, Box<dyn ::std::error::Error>> {
    tracing::debug!("{}", branch.to_string());
    tracing::debug!("{}", commit_hash);

    let response = client
        .get(format!(
            "https://api.github.com/repos/nixos/nixpkgs/compare/{}...{}",
            branch.to_string(),
            commit_hash
        ))
        .send()
        .await?;
    // .json::<PrCompare>()
    // .await?;

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

    // tracing::debug!("{:?}", response);

    // ["behind", "identical"].contains(response.status) instead? (do this later just check if it works first)
    // if response.status == "behind" || response.status == "identical" {
    //     tracing::debug!("In nixpkgs!");
    //     Ok(true)
    // } else {
    //     tracing::debug!("lol no");
    //     Ok(false)
    // }
}
