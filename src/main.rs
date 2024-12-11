use reqwest::Response;

mod github;
#[tokio::main]
async fn main() {
    println!("Hello, world!");
    // github::get_pr_info(32).await;
    let test = github::get_pr_info(345325).await;

    println!("{:?}", test.as_ref().unwrap());

    let test2 = github::compare_branches_api(
	"nixos-unstable", test.unwrap().merge_commit_sha.to_string()).await;

    println!("{:?}", test2.unwrap());

 }
