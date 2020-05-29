use crate::lcs::lcs;
use std::fs::File;
use std::fs::create_dir;
use std::io;
use std::io::Read;
use std::io::Write;
use walkdir::WalkDir;
use indicatif::ProgressBar;
use crate::replace::replace;


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
    let total_files :usize = input.len();
    let pb = ProgressBar::new(total_files as u64);
    let mut total_searched :u64 = 0;
    for file_input in input {
        let total_length: usize = file_input.len();
        if total_length > 4 && !file_input.contains(".txt") {
            continue;
        }
        total_searched += 1;
        pb.set_position(total_searched);
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
    pb.finish_with_message("Common substring extracted");
    common_seq
}

pub fn create_processed_files(input: Vec<String>, cstring: String, out: String) {

    println!("creating {} directory", out);
    match create_dir(out.clone()) {
        Ok(_) => {
            println!("New directory created!!");
        },
        Err(_e) => {
            println!("{} directory creation failed", out.clone());
            return
        }
    };
    let total_files = input.len();
    let mut total_proccessed = 0;
    let pb = ProgressBar::new(total_files as u64);
    for file_input in input {
        let total_length: usize = file_input.len();
        total_proccessed += 1;
        pb.set_position(total_proccessed);
        if total_length > 4 && !file_input.contains(".txt") {
            continue;
        }
        let cloned_file = file_input.clone();
        let file_names : Vec<&str> = cloned_file.split("/").collect();
        let total = file_names.len();
        let mut path_name = format!("{}/{}", out, file_names[total-1]);
        match read_file(file_input) {
            Ok(content_string) => {
                let replaced_content = replace(content_string, cstring.clone());
                if let Ok(mut fl) = File::create(path_name.clone()) {
                    fl.write_all(replaced_content.as_bytes());
                    fl.flush();
                } else {
                    println!("New file failed {:?}", path_name.clone());
                }
            },
            Err(_e) => {

            }
        };
    }
    pb.finish_with_message("Completed!!")
}

