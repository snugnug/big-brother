use sqlx::sqlite;

pub struct TrackedPR {
    pub id: u64,
    pub merged: bool,
    pub merged_into: Vec<String>,
    pub unmerged_into: Vec<String>,
}
