use std::fs;
use std::env;
use std::fs::File;
use std::io::Read;
use std::path::Path;
use sha1::{Sha1, Digest};
use flate2::write::ZlibEncoder;
use flate2::Compression;
use std::io::Write;


pub fn write_blob(content: &Vec<u8>) -> String {
    let blob_content: String = format!("blob {}\0", content.len());
    let mut hasher: sha1::digest::core_api::CoreWrapper<sha1::Sha1Core> = Sha1::new();
    hasher.update(blob_content.as_bytes());
    hasher.update(&content);
    let sha1_hash = hasher.finalize();
    let sha1_hash_str: String = format!("{:x}", sha1_hash);

    let blob_path = format!(".git/objects/{}/{}", &sha1_hash_str[0..2], &sha1_hash_str[2..]);
    fs::create_dir_all(Path::new(&blob_path).parent().unwrap()).expect("Unable to create directory");
    let mut encoder: ZlibEncoder<File> = ZlibEncoder::new(File::create(blob_path).expect("Unable to create file"), Compression::default());
    encoder.write_all(blob_content.as_bytes()).expect("Unable to write to file");
    encoder.write_all(&content).expect("Unable to write to file");

    sha1_hash_str
}