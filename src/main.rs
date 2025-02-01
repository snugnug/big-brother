use clap::Parser;
// use reqwest::Client;
// use sqlx::SqlitePool;
use std::net::Ipv4Addr;

// mod database;
mod github;
mod web;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    // /// Path to store monitor jobs
    // #[arg(short, long, default_value = "/var/lib/big-brother")]
    // datadir: String,
    /// Port to serve the Web Interface on
    #[arg(long, default_value_t = 3000)]
    port: u16,
    /// Host to serve the Web Interface on
    #[arg(long, default_value = "127.0.0.1", value_parser = clap::value_parser!(Ipv4Addr))]
    host: std::net::Ipv4Addr,
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

    // match std::fs::create_dir_all(args.datadir.clone()) {
    //     Ok(_) => {}
    //     Err(err) => {
    //         tracing::error!("Could not create data directory!");
    //         panic!("{}", err);
    //     }
    // };

    // let client = Client::builder()
    //     .user_agent(format!("big-brother {}", env!("CARGO_PKG_VERSION")))
    //     .build()
    //     .unwrap();

    // let db = database::initalize_database(args.datadir).await;
    web::serve(args.host, args.port).await;
}
