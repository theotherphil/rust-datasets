//! Utilities for storing dataset files.

use std::env;
use std::fs;
use std::fs::File;
use std::io::{Read, Write};
use std::path::Path;

use hyper::client::Client;

pub fn data_home(dataset_name: &str) -> String {
    let mut path = env::current_dir().unwrap();
    path.push(format!("data/{}/", dataset_name));
    if !path.is_dir() {
        fs::create_dir(path.as_path());
    }
    String::from(path.to_str().unwrap())
}

pub fn ensure_downloaded(address: &str, destination: &str) {
    if Path::new(&destination).is_file() {
        println!("Already got {}", address);
    } else {
        println!("Downloading {}", address);
        download_to(address, destination);
    }
}

pub fn download_to(address: &str, destination: &str) {
    let client = Client::new();
    println!("Getting {}", address);
    let mut response = client.get(address).send().unwrap();
    let status = response.status;
    if !status.is_success() {
        panic!("Error downloading from {}: {}", address, status);
    }
    let mut body = Vec::new();
    response.read_to_end(&mut body);
    println!("Saving to {}", destination);
    let mut f = File::create(destination).unwrap();
    f.write_all(&body);
}
