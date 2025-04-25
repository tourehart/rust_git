use std::{collections::HashMap, fs::{self, File}, io::{BufRead, BufReader, Read, Write}, path::{Path, PathBuf}};


pub fn create_dir(path: &str,dir:&str){
    let mut full_path = PathBuf::from(path);
    full_path.push(dir);
    fs::create_dir_all(full_path).unwrap();
}

pub fn create_file(path: &str,file:&str){
    let mut full_path = PathBuf::from(path);
    full_path.push(file);
    // 创建文件所在的目录（如果不存在）
    if let Some(parent_dir) = full_path.parent() {
        fs::create_dir_all(parent_dir).unwrap();
    }
    fs::File::create(full_path).unwrap();
}

pub fn delete_file(file_path: &str){
    fs::remove_file(file_path).unwrap();
}

pub fn if_file_exist(file_path: &str)-> bool{
    let path = Path::new(file_path);
    path.exists() && path.is_file()

}

pub fn combine_paths(repo_path: &str, path: &str) -> String {
    if path == "." {
        repo_path.to_string()
    } else {
        let mut combined_path = PathBuf::from(repo_path);
        combined_path.push(path);
        combined_path.to_str().unwrap_or("").to_string()
    }
}

pub fn write(file:String, content:String){
    let mut file = File::create(file).unwrap();
    file.write_all(content.as_bytes()).unwrap();
}

pub fn read(file:String)->String{
    let mut file = File::open(file).unwrap(); 
    let mut content = String::new(); 
    file.read_to_string(&mut content).unwrap();
    content 
}

pub fn read_index(file:String)-> Vec<String>{
    let mut content = String::new();
    let file = File::open(&file).unwrap();
    let reader = BufReader::new(file);
    let lines: Vec<String> = reader.lines().filter_map(|line| line.ok()).collect();
    lines
    // for (index, line) in lines.iter().enumerate() {
    //     content.push_str(line);
    //     if index < lines.len() - 1 {
    //         content.push('\n');
    //     }
    // }
    // content
}

pub fn collect_files(repo_path: &str, base_path: &str, files: &mut HashMap<String, ()>) {
    let full_path = Path::new(repo_path).join(base_path);
    if full_path.is_dir() {
        for entry in fs::read_dir(full_path).unwrap() {
            let entry = entry.unwrap();
            let path = entry.path();
            let relative_path = path.strip_prefix(repo_path).unwrap().to_str().unwrap().to_string();
            if path.is_file() {
                files.insert(relative_path, ());
            } else if path.is_dir() {
               collect_files(repo_path, &relative_path, files);
            }
        }
    }
}
pub fn delete_files(repo_path: &str,  files: &mut HashMap<String, ()>) {
    for (path, _) in files {
        let full_path = Path::new(repo_path).join(path);
        if full_path.exists() {
            if full_path.is_file() {
                fs::remove_file(full_path).unwrap();
            } else if full_path.is_dir() {
                fs::remove_dir_all(full_path).unwrap();
            }
        }
    }
}