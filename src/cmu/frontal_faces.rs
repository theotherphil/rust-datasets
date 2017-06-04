//! The CMU frontal faces dataset.
//!
//! http://vasc.ri.cmu.edu/idb/html/face/frontal_images/

use std::io::BufReader;
use std::io::prelude::*;
use std::fs::File;

use tar::Archive;

use store;

static CMU_FRONTAL_FACES_NAME: &'static str = "CMU_FRONTAL_FACES";
static CMU_FRONTAL_FACES_HOMEPAGE: &'static str = "http://vasc.ri.cmu.edu/idb/images/face/frontal_images/";

fn ensure_downloaded(file_name: &str) -> String {
    let address = CMU_FRONTAL_FACES_HOMEPAGE.to_owned() + file_name;
    let destination = store::data_home(CMU_FRONTAL_FACES_NAME) + file_name;
    store::ensure_downloaded(&address, &destination);
    destination
}

pub fn prepare() {
    // Truth file
    ensure_downloaded("list.html");

    // Tar containing the image files
    let downloaded_tar = ensure_downloaded("images.tar");
    let mut tar_unpack_target = downloaded_tar.clone();
    tar_unpack_target.truncate(downloaded_tar.len() - 4);

    println!("Extracting {} to {}", downloaded_tar, tar_unpack_target);
    let mut archive = Archive::new(File::open(downloaded_tar).unwrap());
    archive.unpack(tar_unpack_target).unwrap();
}
