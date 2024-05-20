use std::fs;
use std::fs::File;
use std::path::Path;
use sha1::{Sha1, Digest};
use flate2::write::ZlibEncoder;
use flate2::Compression;
use std::io::Write;
use std::fs::read_dir;

pub fn write(content: &[u8], g_type: &str) -> (String, Vec<u8>) {
    let mut hasher = Sha1::new();
    hasher.update(format!("{} {}\0", g_type, content.len()).as_bytes());
    hasher.update(content);
    let sha1_hash = hasher.finalize();
    let sha1_hash_str = format!("{:x}", sha1_hash);

    let blob_path = format!(".git/objects/{}/{}", &sha1_hash_str[0..2], &sha1_hash_str[2..]);
    fs::create_dir_all(Path::new(&blob_path).parent().unwrap()).expect("Unable to create directory");
    let mut encoder = ZlibEncoder::new(File::create(blob_path).expect("Unable to create file"), Compression::default());
    encoder.write_all(format!("{} {}\0", g_type, content.len()).as_bytes()).expect("Unable to write to file");
    encoder.write_all(content).expect("Unable to write to file");

    (sha1_hash_str, sha1_hash.to_owned().to_vec())
}

pub fn write_blob(content: &[u8]) -> (String, Vec<u8>) {
    write(content, "blob")
}

pub fn write_tree(path: &str) -> (String, Vec<u8>) {
    let mut entries: Vec<(u32, String, Vec<u8>)> = vec![];

    for entry in read_dir(path).expect("Unable to read directory") {
        let entry = entry.expect("Unable to read directory entry");
        let metadata = entry.metadata().expect("Unable to read metadata");
        let mode = if metadata.is_dir() {
            0o040000
        } else {
            0o100644
        };
        let name = entry.file_name().into_string().expect("Unable to convert OsString to String");
        let path = entry.path();
        if name == ".git" {
            continue;
        }
        let (_, sha1_hash) = if metadata.is_dir() {
            write_tree(path.to_str().expect("Unable to convert PathBuf to str"))
        } else {
            let content = std::fs::read(&path).expect("Unable to read file");
            write_blob(&content)
        };
        entries.push((mode, name, sha1_hash));
    }

    entries.sort_by(|a, b| a.1.cmp(&b.1));

    let mut tree_content = Vec::new();
    for (mode, name, sha1_hash) in entries {
        tree_content.extend_from_slice(format!("{:05o} {}\0", mode, name).as_bytes());
        tree_content.extend_from_slice(&sha1_hash);
    }

    write(&tree_content, "tree")
}