use futures::prelude::*;

use serde_json;
use url::Url;
use dose_types::*;

use std::sync::{Arc, Mutex, RwLock};
use std::sync::mpsc::{channel, Sender, Receiver};
use std::io::{self, Read, Write};
use std::os::unix::net::UnixListener;
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
    downloads: Vec<Option<Dl>>,
}

impl DlServer {
    pub fn new() -> Self {
        DlServer {
            downloads: Vec::new(),
        }
    }

    pub fn eval_request(&mut self, req: Request) -> Box<Future<Item = Response, Error = Response>> {
        unimplemented!()
    }
}