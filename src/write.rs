use std::fs;
use std::env;
use std::fs::File;
use std::io::Read;
use std::path::Path;
use sha1::{Sha1, Digest};
use flate2::write::ZlibEncoder;
use flate2::Compression;
use std::io::Write;


pub fn write_blob(content: &[u8]) -> String {
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

use std::fs::read_dir;
use std::os::unix::fs::PermissionsExt;

pub fn write_tree(path: &str) -> String {
    let mut entries: Vec<(u32, String, String)> = vec![];

    for entry in read_dir(path).expect("Unable to read directory") {
        let entry = entry.expect("Unable to read directory entry");
        let metadata = entry.metadata().expect("Unable to read metadata");
        let permissions = metadata.permissions().mode();
        let name = entry.file_name().into_string().expect("Unable to convert OsString to String");
        let path = entry.path();
        let sha1_hash_str = if metadata.is_dir() {
            write_tree(path.to_str().expect("Unable to convert PathBuf to str"))
        } else {
            let content = std::fs::read(&path).expect("Unable to read file");
            write_blob(&content)
        };
        entries.push((permissions, name, sha1_hash_str));
    }

    entries.sort_by(|a, b| a.1.cmp(&b.1));

    let mut tree_content = String::new();
    for (mode, name, sha1_hash_str) in entries {
        tree_content.push_str(&format!("{} {}\0{}", mode, name, sha1_hash_str));
    }

    write_blob(tree_content.as_bytes())
}