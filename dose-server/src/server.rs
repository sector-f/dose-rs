use tokio_core::reactor::Handle;

use futures::prelude::*;
use futures::future;

use serde_json;
use url::Url;

use dose_types::*;

use std::io::{self, Read, Write};
use std::net::Shutdown;
use std::path::PathBuf;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::thread::spawn;

struct Download {
    url: Url,
    path: PathBuf,
    status: DlStatus,
    bytes_read: u64,
    bytes_total: Option<u64>,
    // time_elapsed
}

pub struct DlServer {
    handle: Handle,
    downloads: Mutex<HashMap<PathBuf, Arc<Mutex<Download>>>>,
}

impl DlServer {
    pub fn new(h: Handle) -> Self {
        DlServer {
            handle: h,
            downloads: Mutex::new(HashMap::new()),
        }
    }

    pub fn eval_request(&mut self, req: Request) -> Response {
        match req {
            // Request::Add { url, path } => {
            //     unimplemented!()
            // },
            _ => Response::Error("Unimplemented".to_string()),
        }
    }
}
