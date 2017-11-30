extern crate serde_json;

extern crate dose_types;
use dose_types::*;

use std::path::PathBuf;

fn main() {
    // Requests

    let add_request = Request::Add { url: String::from("http://www.example.com"), path: PathBuf::from("/path/to/file") };
    println!("Add request:\n{}\n", serde_json::to_string(&add_request).unwrap());

    let cancel_request = Request::Cancel { id: 0 };
    println!("Cancel request:\n{}\n", serde_json::to_string(&cancel_request).unwrap());

    let dl_status_request = Request::DlStatus { id: 0 };
    println!("Download status request:\n{}\n", serde_json::to_string(&dl_status_request).unwrap());

    let server_status_request = Request::ServerStatus;
    println!("Server status request:\n{}\n", serde_json::to_string(&server_status_request).unwrap());

    // Responses

    let dlr1 = DlResponse {
        id: 0,
        url: String::from("http://www.example.com/foo.jpg"),
        path: PathBuf::from("/path/to/foo.jpg"),
        bytes_read: 510,
        bytes_total: Some(1024),
    };

    let dlr2 = DlResponse {
        id: 1,
        url: String::from("http://www.example.com/bar.jpg"),
        path: PathBuf::from("/path/to/bar.jpg"),
        bytes_read: 5,
        bytes_total: Some(10000),
    };

    let status_response = Response::DlStatus(dlr1.clone());
    println!("Download status response:\n{}\n", serde_json::to_string(&status_response).unwrap());

    let server_status = Response::ServerStatus(vec![dlr1, dlr2]);
    println!("Server status response:\n{}\n", serde_json::to_string(&server_status).unwrap());

    let error = Response::Error(String::from("404 file not found"));
    println!("Error response:\n{}", serde_json::to_string(&error).unwrap());
}
