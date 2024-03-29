extern crate serde;
extern crate serde_json;
extern crate url;

extern crate hyper;
extern crate hyper_tls;

extern crate tokio_core;
use tokio_core::reactor::Core;

extern crate tokio_io;
use tokio_io::{io, AsyncRead, AsyncWrite};
use tokio_io::codec::LinesCodec;

extern crate tokio_uds;
use tokio_uds::{UnixListener, UnixStream};

extern crate futures;
use futures::prelude::*;
use futures::future;
use futures::stream;

extern crate libc;
use libc::{umask, mode_t};

use std::path::PathBuf;
use std::io::{BufReader, Read, Write};
use std::process::exit;
use std::net::Shutdown;
use std::thread::spawn;

use std::rc::Rc;
use std::cell::RefCell;

extern crate dose_types;
use dose_types::*;

mod server;
use server::DlServer;

fn main() {
    let version = concat!("dose ", env!("CARGO_PKG_VERSION"));

    let mut core = match Core::new() {
        Ok(core) => core,
        Err(e) => {
            eprintln!("Could not create event loop: {}", e);
            exit(1);
        },
    };

    let old_umask: mode_t;
    unsafe { old_umask = umask(0o177); }
    let socket = match UnixListener::bind("/tmp/dose.socket", &core.handle()) {
        Ok(socket) => socket,
        Err(e) => {
            eprintln!("Could not create socket: {}", e);
            exit(1);
        },
    };
    unsafe { let _ = umask(old_umask); };

    let server = Rc::new(RefCell::new(DlServer::new(core.handle())));
    let handle = core.handle();

    let work = socket.incoming().for_each(move |(connection, _addr)| {
        handle.spawn({
            let server = server.clone();
            let lines = connection.framed(LinesCodec::new());
            let (writer, reader) = lines.split();

            let version_stream = stream::once::<_, _>(Ok(version.to_owned()));
            let reply_stream = reader.map(move |line| {
                let reply = match serde_json::from_str(&line) {
                    Ok(r) => {
                        serde_json::to_string(&server.borrow_mut().eval_request(r)).unwrap()
                    },
                    Err(e) => {
                        serde_json::to_string(&Response::Error(format!("{}", e))).unwrap()
                    }
                };
                println!("{:?}", reply);
                reply
            });

            version_stream.chain(reply_stream).forward(writer).map(|_| ()).map_err(|_| ())
        });

        Ok(())
    })
    .map_err(|_| ());

    core.run(work);
}
