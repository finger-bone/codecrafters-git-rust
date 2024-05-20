use std::fs::File;
use std::io::Read;
use std::path::Path;
use flate2::read::ZlibDecoder;

pub fn get_object_content(object_hash: &str) -> (String, Vec<u8>) {
    let mut file = File::open(Path::new(
        &format!(
            ".git/objects/{}/{}",
            &object_hash[0..2],
            &object_hash[2..])
    )).expect("Unable to open file");
    let mut compressed_content = Vec::new();
    file.read_to_end(&mut compressed_content).expect("Unable to read file");

    let mut decoder = ZlibDecoder::new(&compressed_content[..]);
    let mut decompressed_content = Vec::new();
    decoder.read_to_end(&mut decompressed_content).expect("Unable to decompress");

    // git stores all objects as
    // <type> <size>\0<content>
    let blank_index = decompressed_content.iter().position(|&b| b == b' ').expect("Space not found");
    let null_index = decompressed_content.iter().position(|&b| b == b'\0').expect("Null byte not found");
    let size = std::str::from_utf8(&decompressed_content[blank_index + 1 .. null_index]).unwrap().parse::<usize>().unwrap();

    (
        String::from_utf8_lossy(&decompressed_content[..blank_index]).to_string(),
        decompressed_content[(null_index + 1)..=(null_index + size)].to_vec()
    )
}