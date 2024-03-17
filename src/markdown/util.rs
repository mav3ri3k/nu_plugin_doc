use std::fs;
use std::ffi::OsString;

pub fn dir_walk(path: String) -> Vec<fs::DirEntry> {
    // returns list file and dir names present in path
    let mut entries: Vec<fs::DirEntry> = vec![];

    for entry in fs::read_dir(path).unwrap() {
        entries.push(entry.unwrap())
    }
    entries
}

pub fn file_content(path: OsString) -> String {
    fs::read_to_string(path)
        .expect("Should have been able to read the file")
        .to_string()
}
