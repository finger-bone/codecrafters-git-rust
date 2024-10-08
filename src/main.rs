pub mod retrieve;
pub mod parse;
pub mod write;
pub mod commit;
pub mod clone;

use crate::retrieve::get_object_content;

use std::fs;
use std::env;
use std::fs::File;
use std::io::Read;
use std::path::Path;

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

    let blob_hash = write::write_blob(&content);

    println!("{}", blob_hash.0);
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

pub fn write_tree_handler(_: &Vec<String>) {
    let binding = std::fs::canonicalize(".").expect("Unable to get current directory");
    let path = binding.to_str().expect("Unable to convert PathBuf to str");

    let tree_hash = write::write_tree(path);

    println!("{}", tree_hash.0);
}

pub fn commit_tree_handler(args: &Vec<String>) {
    let tree_hash = &args[2];
    let commit_hash = &args[4];
    let message = &args[6];

    let commit = commit::commit_tree(tree_hash, commit_hash, message);

    println!("{}", commit.0);
}

pub fn clone_handler(args: &Vec<String>) {
    let repo_url = reqwest::Url::parse(&args[2]).expect("Invalid URL");
    let output_path = Path::new(&args[3]);

    clone::clone(&repo_url, Some(output_path));
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
    } else if args[1] == "write-tree" {
        write_tree_handler(&args);
    } else if args[1] == "commit-tree" {
        commit_tree_handler(&args);
    } else if args[1] == "clone" {
        clone_handler(&args);
    } else {
        println!("unknown command: {}", args[1]);
    }
}
