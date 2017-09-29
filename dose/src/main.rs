extern crate serde;
extern crate serde_json;
extern crate reqwest;
extern crate url;

extern crate tokio_core;
use tokio_core::reactor::Core;

extern crate tokio_io;
use tokio_io::{io, AsyncRead};

extern crate tokio_uds;
use tokio_uds::{UnixListener, UnixStream};

extern crate futures;
use futures::{Future, Stream};

extern crate libc;
use libc::{umask, mode_t};

use std::path::PathBuf;
use std::io::{Read, Write};
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
    let mut core = match Core::new() {
        Ok(core) => core,
        Err(e) => {
            eprintln!("Could not create event loop: {}", e);
            exit(1);
        },
    };
    let handle = core.handle();

    let old_umask: mode_t;
    unsafe { old_umask = umask(0o177); }
    let socket = match UnixListener::bind("/tmp/dose.socket", &handle) {
        Ok(socket) => socket,
        Err(e) => {
            eprintln!("Could not create socket: {}", e);
            exit(1);
        },
    };
    unsafe { let _ = umask(old_umask); };

    let server = Rc::new(RefCell::new(DlServer::new()));

    let connections = socket.incoming().for_each(|(stream, _addr)| {
        let server = server.clone();

        handle.spawn({
            let (reader, writer) = stream.split();

            io::copy(reader, writer).then(|_| Ok(()))

            // io::read_to_end(reader, Vec::new())
            // .map(|(_reader, buffer)| {
            //     serde_json::from_slice(buffer)
            // })

            // io::read_to_end(reader, Vec::new())
            // .map(|(_, buffer)| String::from_utf8(buffer))
            // .map_err(|e| Response::Error(e.to_string())) // io::Error
            // .then(move |result| {
            //     let response = match result {
            //         Ok(s_result) => {
            //             match s_result {
            //                 Ok(string) => {
            //                     match serde_json::from_str::<Request>(&string) {
            //                         Ok(request) => {
            //                             let mut server = server.borrow_mut();
            //                             server.process(request)
            //                         },
            //                         Err(ser_e) => {
            //                             Response::Error(ser_e.to_string())
            //                         },
            //                     }
            //                 },
            //                 Err(utf_e) => {
            //                     Response::Error(utf_e.to_string())
            //                 },
            //             }
            //         },
            //         Err(io_err) => {
            //             io_err
            //         }
            //     };

            //     io::write_all(writer, serde_json::to_string(&response).unwrap());
            //     Ok(())
            // })
        });

        Ok(())
    });

    // let _ = core.run(connections);
}

// fn run(stream: &mut UnixStream, server: &mut DlServer) {
//     let mut buf = String::new();
//     let _ = stream.read_to_string(&mut buf);

//     let mut json_response: String;
//     match serde_json::from_str::<Request>(&buf) {
//         Ok(req) => {
//             let response = server.process(req);
//             json_response = serde_json::to_string(&response).unwrap();
//         },
//         Err(e) => {
//             let msg = e.to_string();
//             let response = Response::Error(&msg);
//             json_response = serde_json::to_string(&response).unwrap();
//         },
//     }

//     let _ = stream.write_all(json_response.as_bytes());
// }
