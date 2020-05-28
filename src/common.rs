use crate::lcs::lcs;
use std::fs::File;
use std::io;
use std::io::Read;
use walkdir::WalkDir;

pub fn get_text_files(dirname: String) -> Vec<String> {
    let mut paths: Vec<String> = Vec::new();
    for entry in WalkDir::new(&dirname).into_iter().filter_map(|e| e.ok()) {
        if entry.file_type().is_file() {
            paths.push(entry.path().to_str().unwrap().to_owned());
        }
    }
    return paths;
}

pub fn read_file(path: String) -> io::Result<String> {
    let mut content: String = String::new();
    match File::open(path) {
        Ok(mut file) => {
            file.read_to_string(&mut content);
            Ok(content)
        }
        Err(e) => Err(e),
    }
}

pub fn get_substring_from_files(input: Vec<String>) -> String {
    let mut common_seq: String = String::new();
    for file_input in input {
        let total_length: usize = file_input.len();
        if total_length > 4 && !file_input.contains(".txt") {
            continue;
        }
        match read_file(file_input) {
            Ok(content) => {
                if common_seq.is_empty() {
                    common_seq = content.clone();
                } else {
                    common_seq = lcs(common_seq, content);
                }
            }
            Err(_) => {}
        };
    }
    common_seq
}
