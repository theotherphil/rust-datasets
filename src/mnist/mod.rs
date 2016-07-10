//
// The MNIST database of handrwritten digits.
//
// http://yann.lecun.com/exdb/mnist/
//

use std::convert::AsMut;
use std::env;
use std::io::BufReader;
use std::io::prelude::*;
use std::fs;
use std::fs::File;
use std::mem;
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
    let archive = File::open(archive).unwrap();
    let mut reader = BufReader::new(archive);
    let mut bytes = Vec::new();
    let _ = reader.read_to_end(&mut bytes).unwrap();
    let mut d = GzDecoder::new(bytes.as_slice()).unwrap();
    let mut res = Vec::new();
    d.read_to_end(&mut res).unwrap();
    res
}

// Thanks to: http://stackoverflow.com/a/37682288/2050
fn clone_into_array<A, T>(slice: &[T]) -> A
    where A: Sized + Default + AsMut<[T]>,
          T: Clone {
    let mut a: A = Default::default();
    <A as AsMut<[T]>>::as_mut(&mut a).clone_from_slice(slice);
    a
}

fn check_idx_integrity(data: Vec<u8>) -> bool {
    let magic;
    unsafe {
        magic = u32::from_be(mem::transmute::<[u8; 4], u32>(clone_into_array(&data[0..4])));
    }
    println!("Magic number: {}", magic);
    if magic == 2051 {
        check_idx_integrity_image(data)
    } else if magic == 2049 {
        check_idx_integrity_label(data)
    } else {
        false
    }
}

fn check_idx_integrity_image(data: Vec<u8>) -> bool {
    let (count, rows, cols);
    unsafe {
        count = u32::from_be(mem::transmute::<[u8; 4], u32>(clone_into_array(&data[4..8])));
        rows = u32::from_be(mem::transmute::<[u8; 4], u32>(clone_into_array(&data[8..12])));
        cols = u32::from_be(mem::transmute::<[u8; 4], u32>(clone_into_array(&data[12..16])));
    }
    println!("Image blob: {}x{}x{}", count, rows, cols);
    (data.len() - 16) == ((count * rows * cols) as usize)
}

fn check_idx_integrity_label(data: Vec<u8>) -> bool {
    let count;
    unsafe {
        count = u32::from_be(mem::transmute::<[u8; 4], u32>(clone_into_array(&data[4..8])));
    }
    println!("Label blob: {}", count);
    (data.len() - 8) == (count as usize)
}

pub fn prepare() {
    let archives = [
        "train-labels-idx1-ubyte.gz",
        "train-images-idx3-ubyte.gz",
        "t10k-labels-idx1-ubyte.gz",
        "t10k-images-idx3-ubyte.gz"
    ];
    for a in archives.iter() {
        ensure_downloaded(a);
    }
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
