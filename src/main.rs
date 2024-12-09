use reqwest::Response;

mod github;
#[tokio::main]
async fn main() {
    println!("Hello, world!");
    // github::get_pr_info(32).await;
    let test = github::get_pr_info(32).await;

    println!("{:?}", test);
 }
