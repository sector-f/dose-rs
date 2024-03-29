use hyper::Client;
use hyper::client::HttpConnector;
use hyper_tls::HttpsConnector;

use tokio_core::reactor::Handle;

use futures::prelude::*;
use futures::future;
use futures::sync::mpsc::{self, Sender, Receiver};

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
    rx:  Receiver<Message>, // I definitely need a Receiver<Message> but this may be the wrong place for it
    url: Url,
    path: PathBuf,
    status: DlStatus,
    bytes_read: u64,
    bytes_total: Option<u64>,
    // time_elapsed
}

pub struct DlServer {
    handle: Handle,
    client: Client<HttpsConnector<HttpConnector>>,
    downloads: Mutex<HashMap<PathBuf, Arc<Mutex<Download>>>>,
}

impl DlServer {
    pub fn new(h: Handle) -> Self {
        let client = Client::configure()
            .connector(HttpsConnector::new(4, &h).unwrap()) // Maybe make this fn return Result?
            .build(&h);

        DlServer {
            handle: h,
            client: client,
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

enum Message {
    Cancel
}
