pub const DIGEST_LEN: usize = 20;

pub struct TreeEntry {
    pub mode: u32,
    pub name: String,
    pub hash: Vec<u8>,
}


pub fn parse_tree(content: &Vec<u8>) -> Vec<TreeEntry> {
    let mut entries = vec![];
    let mut cur = 0;
    while cur < content.len() {
        let next_space = content[cur..].iter().position(|&b| b == b' ').expect("Space not found") + cur;
        let next_null = content[next_space..].iter().position(|&b| b == b'\0').expect("Null byte not found") + next_space;
        let mode = std::str::from_utf8(&content[cur..next_space]).unwrap().parse::<u32>().unwrap();
        let name = std::str::from_utf8(&content[next_space + 1..next_null]).unwrap().to_string();
        let hash_end = next_null + 1 + DIGEST_LEN;
        let hash = content[next_null + 1..hash_end].to_vec();
        
        eprintln!("mode: {}, name: {}, hash: {:?}", mode, name, hash);
        
        entries.push(TreeEntry { mode, name, hash });
        
        cur = next_null + 1 + DIGEST_LEN;
    }
    entries
}