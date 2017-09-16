#[macro_use] extern crate serde_derive;

use std::path::Path;

#[derive(Serialize, Deserialize)]
pub enum Request<'a> {
    Add { url: &'a str, path: &'a Path },
    Cancel { id: usize },
    Remove { id: usize },
    DlStatus { id: usize },
    ServerStatus,
}

#[derive(Serialize, Deserialize)]
pub enum Response<'a> {
    DlStatus(DlResponse<'a>),
    ServerStatus(Vec<DlResponse<'a>>),
    Error(&'a str),
}

#[derive(Serialize, Deserialize)]
pub enum DlStatus {
    InProgress,
    Completed,
    Canceled,
    Failed,
}

#[derive(Serialize, Deserialize, Clone, Copy)]
pub struct DlResponse<'a> {
    pub id: usize,
    pub url: &'a str,
    pub path: &'a Path,
    pub percent: f32,
}
