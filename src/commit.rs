use crate::write::write;

pub fn commit_tree(
    tree_hash: &str,
    commit_hash: &str,
    message: &str,
) -> (String, Vec<u8>) {
    let mut commit_content = format!("tree {}\n", tree_hash);
    commit_content.push_str(&format!("parent {}\n", commit_hash));
    commit_content.push_str(&format!("author {} <> {}\n", "author", "1709990458 +0100"));
    commit_content.push_str(&format!("committer {} <> {}\n", "committer", "1709990458 +0100"));
    commit_content.push_str(&format!("\n{}\n", message));

    write(commit_content.as_bytes(), "commit")
}