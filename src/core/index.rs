use std::{collections::HashMap, path::Path};
use crate::utils::fs;

// 索引结构体定义
pub struct Index {
    repo_path: String, 
   staged_files: HashMap<String, String> // 暂存文件集合
}

impl Index {
    /// 加载现有索引
    pub fn load(repo_path: &str) -> Self {
        let index_file = format!("{}/.git/index", repo_path);

        let staged_files = if Path::new(&index_file).exists() {
            let content = fs::read_index(index_file);
            let mut set = HashMap::new();
            for line in content {
                let parts: Vec<&str> = line.split_whitespace().collect();
                set.insert(parts[2].to_string(),parts[2].to_string());
            }
            set
        } else {
            HashMap::new()
        };

        Index {
            repo_path: repo_path.to_string(),
            staged_files
        }
    }

    /// 添加文件到暂存区
    pub fn stage_file(&mut self, path: &str, hash: String) {
        self.staged_files.insert(hash,path.to_string());
        self.persist();
    }
    
    /// 从暂存区移除文件
    pub fn unstage_file(&mut self, path: &str) {
        self.staged_files.remove(path);
        self.persist();
    }

    /// 内部保存方法
    pub fn persist(&self) {
        let index_file = format!("{}/.git/index", self.repo_path);
        let content = self.staged_files.iter()
        .map(|(key, value)| format!("blob {} {}\n", value, key))
        .collect::<String>();
        fs::write(index_file, content);
    }

    pub fn get_staged_files(&self) -> HashMap<String, String> {
        self.staged_files.clone()
    }
}
   