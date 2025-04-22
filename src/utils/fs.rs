use std::{fs::{self, File}, io::{BufRead, BufReader, Read, Write}, path::{Path, PathBuf}};


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