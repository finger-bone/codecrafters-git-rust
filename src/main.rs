use std::fs;
use std::env;
use std::fs::File;
use std::io::Read;
use std::path::Path;
use flate2::read::ZlibDecoder;
use sha1::{Sha1, Digest};
use flate2::write::ZlibEncoder;
use flate2::Compression;
use std::io::Write;

fn cat_file_handler(args: &Vec<String>) {
    let blob_sha = &args[3];
    let blob_path = format!(".git/objects/{}/{}", &blob_sha[0..2], &blob_sha[2..]);

    let mut file = File::open(Path::new(&blob_path)).expect("Unable to open file");
    let mut compressed_content = Vec::new();
    file.read_to_end(&mut compressed_content).expect("Unable to read file");

    let mut decoder = ZlibDecoder::new(&compressed_content[..]);
    let mut decompressed_content = Vec::new();
    decoder.read_to_end(&mut decompressed_content).expect("Unable to decompress");

    let null_index = decompressed_content.iter().position(|&b| b == b'\0').expect("Null byte not found");
    let content = &decompressed_content[(null_index + 1)..];

    print!("{}", String::from_utf8_lossy(content));
}

fn hash_object_handler(args: &Vec<String>) {
    let file_path = &args[3];
    let mut file = File::open(file_path).expect("Unable to open file");
    let mut content = Vec::new();
    file.read_to_end(&mut content).expect("Unable to read file");

    let blob_content = format!("blob {}\0", content.len());
    let mut hasher = Sha1::new();
    hasher.update(blob_content.as_bytes());
    hasher.update(&content);
    let sha1_hash = hasher.finalize();
    let sha1_hash_str = format!("{:x}", sha1_hash);

    let blob_path = format!(".git/objects/{}/{}", &sha1_hash_str[0..2], &sha1_hash_str[2..]);
    let mut encoder = ZlibEncoder::new(File::create(blob_path).expect("Unable to create file"), Compression::default());
    encoder.write_all(blob_content.as_bytes()).expect("Unable to write to file");
    encoder.write_all(&content).expect("Unable to write to file");

    println!("{}", sha1_hash_str);
}

fn main() {
    // You can use print statements as follows for debugging, they'll be visible when running tests.
    // eprintln!("Logs from your program will appear here!");

    // Uncomment this block to pass the first stage
    let args: Vec<String> = env::args().collect();
    if args[1] == "init" {
        fs::create_dir(".git").unwrap();
        fs::create_dir(".git/objects").unwrap();
        fs::create_dir(".git/refs").unwrap();
        fs::write(".git/HEAD", "ref: refs/heads/main\n").unwrap();
        println!("Initialized git directory")
    } else if args[1] == "cat-file" {
        cat_file_handler(&args);
    } else if args[2] == "hash-object" {
        hash_object_handler(&args);
    } else {
        println!("unknown command: {}", args[1]);
    }
}
