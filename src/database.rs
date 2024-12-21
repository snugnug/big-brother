use sqlx::{migrate::MigrateDatabase, prelude::FromRow, sqlite::{Sqlite, SqlitePoolOptions}, Pool};

#[derive(FromRow)]
pub struct PullRequest {
    pub id: u64,
    pub merged: bool,
    pub merged_into: Vec<String>,
    pub unmerged_into: Vec<String>,
}

pub async fn initalize_database(path: String) -> Pool<Sqlite> {
    let dbpath: String = path.clone() + "/database.db";
    
    println!("{}", path);
    println!("{}", dbpath.clone());

    if std::path::Path::new(dbpath.clone().as_str()).exists() {
	println!("Database exists, skipping initalization...");
    } else {
 
    Sqlite::create_database(
	format!(
	    "sqlite:{}",
	    dbpath.as_str()
	)
	    .as_str(),
    )
	.await
	.expect("Failed to create database");
    };
    
    let db = SqlitePoolOptions::new()
        .connect(dbpath.as_str())
        .await
	.unwrap();

    sqlx::migrate!("./migrations").run(&db).await.unwrap();

    return db;
}
