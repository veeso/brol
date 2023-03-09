use std::{env, path::PathBuf};

fn main() {
    let path = match env::args().nth(1) {
        Some(s) => PathBuf::from(s),
        None => panic!("usage: <path_to_file>"),
    };

    let root =
        PathBuf::from("/Users/christianvisintin/Sviluppo/opensource/brol/rust/relative_fs_access/");
    let path = root.join(path);
    println!("Path is {}", path.display());
    //assert!(path.is_relative());
    assert!(path.exists());
}
