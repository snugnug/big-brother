use reqwest::Client;
use sqlx::SqlitePool;
use clap::Parser;

mod github;
mod database;
mod web;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Path to store monitored data
    #[arg(short, long, default_value = "/var/lib/big-brother")]
    datadir: String,
}

#[tokio::main]
async fn main() {

    let subscriber = tracing_subscriber::fmt()
    // Use a more compact, abbreviated log format
	.compact()
    // Display source code file paths
	.with_file(true)
    // Display source code line numbers
	.with_line_number(true)
    // Don't display the event's target (module path)
	.with_target(false)
    // Build the subscriber
	.with_max_level(if cfg!(debug_assertions) {
	    tracing::Level::DEBUG
	} else {
	    tracing::Level::INFO
	})
	.finish();

    match tracing::subscriber::set_global_default(subscriber) {
	Ok(_) => {}
	Err(err) => {
	    tracing::error!("Could not create logger!");
	    panic!("{}", err);
	}
    };
	
    let args = Args::parse();

    match std::fs::create_dir_all(args.datadir.clone()) {
        Ok(_) => {}
        Err(err) => {
	    tracing::error!("Could not create data directory!");
            panic!("{}", err);
        }
    };

    let client = Client::builder()
        .user_agent(format!("big-brother {}", env!("CARGO_PKG_VERSION")))
        .build()
        .unwrap();

    let db = database::initalize_database(args.datadir).await;

    web::serve_web().await;

    // let test = github::get_pr_info(client.clone(), 345325).await;

    // tracing::info!("{:?}", test.as_ref().unwrap());

    // let test2 = github::compare_branches_api(client,
    // 	"nixos-unstable", test.unwrap().merge_commit_sha.to_string()).await;

    // tracing::info!("{:?}", test2.unwrap());

 }
