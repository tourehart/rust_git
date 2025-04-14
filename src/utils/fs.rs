use std::{fs::File, io::{BufRead, BufReader, Write}, path::PathBuf};


pub fn create_dir(path: &str,dir:&str){

}

pub fn create_file(path: &str,file:&str){

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

pub fn read(file:String)-> String{
    let mut content = String::new();
    let file = File::open(&file).unwrap();
    let reader: BufReader<File> = BufReader::new(file);
    for line in reader.lines() {
        if let Ok(line_content) = line {
            content.push_str(&line_content);
            content.push('\n');
        }
    }
    content
}