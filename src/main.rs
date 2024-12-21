use reqwest::Client;
use sqlx::SqlitePool;
use clap::Parser;

mod github;
mod database;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Path to store monitored data
    #[arg(short, long, default_value = "/var/lib/big-brother")]
    datadir: String,
}

#[tokio::main]
async fn main() {

    let args = Args::parse();

    match std::fs::create_dir_all(args.datadir.clone()) {
        Ok(_) => {}
        Err(err) => {
            panic!("Could not create data directory! {}", err);
        }
    };

    println!("Hello, world!");
    let client = Client::builder()
        .user_agent(format!("big-brother {}", env!("CARGO_PKG_VERSION")))
        .build()
        .unwrap();

    println!("Creating database");
    let db = database::initalize_database(args.datadir).await;

    // github::get_pr_info(32).await;
    // let test = github::get_pr_info(client.clone(), 345325).await;

    // println!("{:?}", test.as_ref().unwrap());

    // let test2 = github::compare_branches_api(client,
    // 	"nixos-unstable", test.unwrap().merge_commit_sha.to_string()).await;

    // println!("{:?}", test2.unwrap());

 }
