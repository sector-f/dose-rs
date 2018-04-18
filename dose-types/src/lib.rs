#[macro_use] extern crate serde_derive;

use std::path::PathBuf;

#[derive(Serialize, Deserialize)]
pub enum Request {
    Add { url: String, path: PathBuf },
    Cancel { path: PathBuf },
    Remove { path: PathBuf },
    Info { path: PathBuf },
    ServerInfo,
}

#[derive(Serialize, Deserialize)]
pub enum Response {
    Added { path: PathBuf },
    Canceled { path: PathBuf },
    Info(DlResponse),
    ServerInfo(Vec<DlResponse>),
    Error(String),
}

#[derive(Serialize, Deserialize)]
pub enum DlStatus {
    InProgress,
    Completed,
    Canceled,
    Failed,
}

#[derive(Serialize, Deserialize)]
pub struct DlResponse {
    pub url: String,
    pub path: PathBuf,
    pub status: DlStatus,
    pub bytes_read: u64,
    pub bytes_total: Option<u64>,
    // time_elapsed
}
