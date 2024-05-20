mod retrieve;
pub mod parse;
pub mod write;
use crate::retrieve::get_object_content;

use std::fs;
use std::env;
use std::fs::File;
use std::io::Read;
use std::path::Path;
use sha1::{Sha1, Digest};
use flate2::write::ZlibEncoder;
use flate2::Compression;
use std::io::Write;

fn cat_file_handler(args: &Vec<String>) {
    let blob_hash = &args[3];

    let (object_type, content) = get_object_content(blob_hash);

    assert_eq!(object_type, "blob", "Wrong object type: {}", object_type);

    print!("{}", std::str::from_utf8(&content).unwrap());
}

fn hash_object_handler(args: &Vec<String>) {
    let file_path = &args[3];
    let mut file = File::open(file_path).expect("Unable to open file");
    let mut content: Vec<u8> = Vec::new();
    file.read_to_end(&mut content).expect("Unable to read file");

    let hash = write::write_blob(&content);

    println!("{}", hash);
}

fn ls_tree_handler(args: &Vec<String>) {
    let tree_sha = &args[3];

    let (object_type, content) = get_object_content(tree_sha);

    assert_eq!(object_type, "tree", "Wrong object type: {}", object_type);

    let entries = parse::parse_tree(&content);

    for entry in entries {
        println!("{}", entry.name);
    }
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
    } else if args[1] == "hash-object" {
        hash_object_handler(&args);
    } else if args[1] == "ls-tree" {
        ls_tree_handler(&args);
    } else {
        println!("unknown command: {}", args[1]);
    }
}
