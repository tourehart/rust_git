use crate::utils::fs::{combine_paths, delete_file, if_file_exist, read, write};


pub struct Reference;

impl Reference {
    /// 创建引用文件
    pub fn create(repo_path: &str, ref_name: &str, target_hash: &str) {
        // 实现逻辑：
        //1. 构建引用路径：".git/refs/[heads|tags]/[name]"
        //2. 将哈希值写入目标文件
        //3. 错误时panic（实际项目应返回Result）

        // 构建引用路径：".git/refs/[heads|tags]/[name]"
        let ref_type = if ref_name.starts_with("refs/heads/") || ref_name.starts_with("refs/tags/") {
            ref_name.split('/').nth(1).unwrap()
        } else {
            "heads" // 默认使用heads
        };
        let ref_path = format!("{}/.git/refs/{}/{}", repo_path, ref_type, ref_name.split('/').last().unwrap());
        //将哈希值写入目标文件
        write(ref_path, target_hash.to_string());
    }
    pub fn change_head(repo_path: &str,content: &str){
        let head_path = format!("{}/.git/HEAD", repo_path);
        write(head_path, content.to_string());
    }
    pub fn get_current_hash(repo_path: &str) -> String{
        let binding = read(combine_paths(repo_path, ".git/HEAD"));
        let part: Vec<&str> = binding.split(' ').collect();
        if part.len()== 0 {
            part[0].to_string()
        }
        else{
            Self::resolve(repo_path,part[1])
        }
    }
    /// 删除引用文件
    pub fn delete(repo_path: &str, ref_name: &str) {
        // 实现逻辑：
        //1. 定位引用文件路径
        //2. 删除文件系统对应文件

        let ref_type = if ref_name.starts_with("refs/heads/") || ref_name.starts_with("refs/tags/") {
            ref_name.split('/').nth(1).unwrap()
        } else {
            "heads" // 默认使用heads
        };
        let ref_path = format!("{}/.git/refs/{}/{}", repo_path, ref_type, ref_name.split('/').last().unwrap());

        delete_file(&ref_path);

    }

    /// 解析引用内容
    pub fn resolve(repo_path: &str, ref_name: &str) -> String {
        // 实现逻辑：
        // if 引用文件存在 {
        //     //读取文件内容并去除空白字符
        //     Some(哈希字符串)
        // } else {
        //     None
        // }   
        // 1. 构建引用路径：".git/refs/[heads|tags]/[name]"
        let ref_type = if ref_name.starts_with("refs/heads/") || ref_name.starts_with("refs/tags/") {
            ref_name.split('/').nth(1).unwrap()
        } else {
            "heads" // 默认使用heads
        };
        let ref_path = format!("{}/.git/refs/{}/{}", repo_path, ref_type, ref_name.split('/').last().unwrap());
        // 2. 检查文件是否存在并读取内容
        if if_file_exist(&ref_path) {
            read(ref_path)
        } else {
            "".to_string()
        }
    }

    pub fn update_commit(repo_path: &str, commit_hash: &str){
        if Self::is_detached_head(repo_path){
            Self::change_head(repo_path, commit_hash);
        }
        else{
            let head_path = format!("{}/.git/HEAD", repo_path);
            let binding = read(head_path);
            let part: Vec<&str> = binding.split(' ').collect();
            write(part[1].to_string(), commit_hash.to_string());
        }
    }

    pub fn update_head(repo_path: &str, target_hash: &str){

    }

    pub fn is_detached_head(repo_path: &str) -> bool {
        let head_path = format!("{}/.git/HEAD", repo_path);
        let binding = read(head_path);
        let part: Vec<&str> = binding.split(' ').collect();
        if part.len()== 0 {
            true
        }
        else{
            false
        }
    }
 }
