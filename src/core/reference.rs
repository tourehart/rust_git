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

    pub fn get_current_ref(repo_path: &str) -> String{
        let binding = read(combine_paths(repo_path, ".git/HEAD"));
        let part: Vec<&str> = binding.split(' ').collect();
        part[1].to_string()
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
    pub fn resolve(repo_path: &str, ref_name: &str) -> Option<String> {
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
           Some(read(ref_path))
        } else {
            None
        }
    }
    pub fn update_head(repo_path: &str, target_hash: &str){
        let head_path = format!("{}/.git/HEAD", repo_path);
        
    }

    pub fn update_commit(repo_path: &str, target_hash: &str, ref_name: &str) {
        let head_path = format!("{}/.git/HEAD", repo_path);
        let head_content = format!("ref :refs/heads/{}", ref_name);
        write(head_path, head_content);
        let last_file = format!("{}/.git/refs/heads/{}", repo_path,ref_name);
        write(last_file, target_hash.to_owned());    
    }
 }
