//
// The MNIST database of handrwritten digits.
//
// http://yann.lecun.com/exdb/mnist/
//

use std::env;
use std::io::BufReader;
use std::io::prelude::*;
use std::fs;
use std::fs::File;
use std::path::Path;

use hyper::client::Client;
use flate2::read::GzDecoder;

fn data_home() -> String {
    let mut path = env::current_dir().unwrap();
    path.push("data/");
    if ! path.is_dir() {
        fs::create_dir(path.as_path());
    }
    String::from(path.to_str().unwrap())
}

fn ensure_downloaded(address: &str) {
    println!("Data home {}", data_home());
    let target = data_home() + address;
    if Path::new(&target).is_file() {
        println!("Already got {}", address);
    } else {
        println!("Downloading {}", address);
        let mnist_home = String::from("http://yann.lecun.com/exdb/mnist/");
        download_to(&(mnist_home + address), &target);
    }
}

fn download_to(address: &str, destination: &str) {
    let client = Client::new();
    println!("Getting {}", address);
    let mut archive = client.get(address).send().unwrap();
    let mut body = Vec::new();
    archive.read_to_end(&mut body);
    println!("Saving to {}", destination);
    let mut f = File::create(destination).unwrap();
    f.write_all(&body);
}

fn decompress(archive: &str) -> Vec<u8> {
    let mut archive = File::open(archive).unwrap();
    let mut reader = BufReader::new(archive);
    let mut bytes = Vec::new();
    let _ = reader.read_to_end(&mut bytes).unwrap();
    let mut d = GzDecoder::new(bytes.as_slice()).unwrap();
    let mut res = Vec::new();
    d.read_to_end(&mut res).unwrap();
    res
}

fn check_idx_integrity(data: Vec<u8>) -> bool {
    // TODO
    true
}

pub fn prepare() {
    let train_labels = "train-labels-idx1-ubyte.gz";
    //let train_images = "train-images-idx3-ubyte.gz";
    //let test_labels = "t10k-labels-idx1-ubyte.gz";
    //let test_images = "t10k-images-idx3-ubyte.gz";
    ensure_downloaded(train_labels);
    for res in fs::read_dir(data_home()).unwrap() {
        let entry = res.unwrap().path();
        let path = entry.to_str().unwrap();
        println!("Processing {}", path);
        let data = decompress(path);
        if check_idx_integrity(data) {
            println!("IDX OK");
        } else {
            println!("IDX invalid");
        }
    }
}
