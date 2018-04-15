extern crate serde;
extern crate serde_json;

extern crate clap;
use clap::{App, Arg, SubCommand};

extern crate dose_types;
use dose_types::*;

use std::ffi::OsStr;
use std::io::{BufReader, BufRead, Read, Write};
use std::path::PathBuf;
use std::process::exit;
use std::os::unix::net::UnixStream;
use std::time::Duration;

fn main() {
    let matches = App::new(env!("CARGO_PKG_NAME"))
        .version(env!("CARGO_PKG_VERSION"))
        .about("Command-line client for dose")
        .arg(Arg::with_name("socket")
             .short("s")
             .long("socket")
             .help("Specify the Unix Domain Socket to connect to (default: /tmp/dose.socket)")
             .takes_value(true))
        .subcommand(SubCommand::with_name("download")
                    .about("Adds a new file to be downloaded")
                    .arg(Arg::with_name("url")
                         .help("The file's URL")
                         .index(1)
                         .required(true)
                         .takes_value(true))
                    .arg(Arg::with_name("path")
                         .help("The location to save the file at")
                         .index(2)
                         .required(true)
                         .takes_value(true)))
        .get_matches();

    let request: Request =
        match matches.subcommand() {
            ("download", Some(args)) => {
                let url = args.value_of("url").unwrap();
                let path = args.value_of_os("path").unwrap();
                Request::Add { url: url.to_string(), path: PathBuf::from(path) }
            },
            _ => {
                println!("{}", matches.usage());
                exit(1);
            },
        };

    let path = matches.value_of_os("socket").unwrap_or(OsStr::new("/tmp/dose.socket"));
    let mut socket = match UnixStream::connect(path) {
        Ok(conn) => conn,
        Err(e) => {
            eprintln!("Failed to connect to socket: {}", e);
            exit(1);
        },
    };

    let five_seconds = Duration::from_secs(5);
    let _ = socket.set_read_timeout(Some(five_seconds));
    let _ = socket.set_write_timeout(Some(five_seconds));

    let reader = BufReader::new(socket.try_clone().unwrap());
    let mut lines = reader.lines();

    if let Some(Ok(s)) = lines.next() {
        if s == "dose 0.1.0" {
            socket.try_clone().unwrap().write(serde_json::to_string(&request).unwrap().as_bytes());
        }
    }

    // match &lines.next() {
    //     Some(Ok(s)) if s == "dose 0.1.0" => {

    //     },
    //     _ => {

    //     },
    // }
}
