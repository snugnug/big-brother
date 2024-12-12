use reqwest::Client;

mod github;
#[tokio::main]
async fn main() {
    println!("Hello, world!");
    let client = Client::builder()
        .user_agent(format!("big-brother {}", env!("CARGO_PKG_VERSION")))
        .build()
        .unwrap();

    // github::get_pr_info(32).await;
    let test = github::get_pr_info(client.clone(), 345325).await;

    println!("{:?}", test.as_ref().unwrap());

    let test2 = github::compare_branches_api(client,
	"nixos-unstable", test.unwrap().merge_commit_sha.to_string()).await;

    println!("{:?}", test2.unwrap());

 }
