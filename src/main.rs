use std::fs;
use std::env;
use std::fs::File;
use std::io::Read;
use std::path::Path;
use flate2::read::ZlibDecoder;

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

fn main() {
    // You can use print statements as follows for debugging, they'll be visible when running tests.
    println!("Logs from your program will appear here!");

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
    } else {
        println!("unknown command: {}", args[1])
    }
}
