extern crate serde;
extern crate serde_json;
extern crate reqwest;
extern crate url;

extern crate daemonize;
use daemonize::Daemonize;

extern crate libc;
use libc::{umask, mode_t};

use std::path::PathBuf;
use std::os::unix::net::UnixListener;
use std::io::{Read, Write};
use std::sync::{Arc, Mutex, RwLock};
use std::process::exit;
use std::net::Shutdown;
use std::thread::spawn;

extern crate dose_types;
use dose_types::*;

mod server;
use server::DlServer;

fn main() {
    let old_umask: mode_t;
    let socket_path = PathBuf::from("/tmp/dose.socket");

    unsafe { old_umask = umask(0o177); }
    let socket = match UnixListener::bind(socket_path) {
        Ok(socket) => socket,
        Err(e) => {
            eprintln!("Could not create socket: {}", e);
            exit(1);
        },
    };
    unsafe { let _ = umask(old_umask); };


    let daemon = Daemonize::new()
        .privileged_action(move || {
            let server = DlServer::new();
            run(&socket, server);
        });

    let _ = daemon.start();
}

fn run(socket: &UnixListener, server: DlServer) {
    let server = Arc::new(RwLock::new(server));
    for conn in socket.incoming() {
        let server = server.clone();
        if let Ok(mut stream) = conn {
            spawn(move || {
                let mut buf = String::new();
                let _ = stream.read_to_string(&mut buf);

                let json_response: String;
                match serde_json::from_str::<Request>(&buf) {
                    Ok(req) => {
                        let mut write_guard = server.write().unwrap();
                        let response = write_guard.process(req);
                        json_response = serde_json::to_string(&response).unwrap();
                    },
                    Err(e) => {
                        let msg = e.to_string();
                        let response = Response::Error(&msg);
                        json_response = serde_json::to_string(&response).unwrap();
                    },
                }

                let _ = stream.write_all(json_response.as_bytes());
                let _ = stream.shutdown(Shutdown::Both);
            });
        }
    }
}
