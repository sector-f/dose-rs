use tokio_core::reactor::Handle;

use futures::prelude::*;
use futures::future;

use serde_json;
use url::Url;

use dose_types::*;

use std::io::{self, Read, Write};
use std::net::Shutdown;
use std::path::PathBuf;
use std::thread::spawn;

struct Dl {
    url: Url,
    path: PathBuf,
    status: DlStatus,
    bytes_read: u64,
    bytes_total: u64,
}

pub struct DlServer {
    handle: Handle,
    downloads: Vec<Option<Dl>>,
}

impl DlServer {
    pub fn new(h: Handle) -> Self {
        DlServer {
            handle: h,
            downloads: Vec::new(),
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
