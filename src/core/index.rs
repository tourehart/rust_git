use std::{collections::HashSet, path::Path};
use crate::utils::fs;

// 索引结构体定义
pub struct Index {
    repo_path: String, 
   staged_files: HashSet<String> // 暂存文件集合
}

impl Index {
    /// 加载现有索引
    pub fn load(repo_path: &str) -> Self {
        let index_file = format!("{}/.git/index", repo_path);

        let staged_files = if Path::new(&index_file).exists() {
            let content = fs::read(index_file);
            let mut set = HashSet::new();
            for line in content.lines() {
                set.insert(line.to_string());
            }
            set
        } else {
            HashSet::new()
        };

        Index {
            repo_path: repo_path.to_string(),
            staged_files
        }
    }

    /// 添加文件到暂存区
    pub fn stage_file(&mut self, path: &str) {
        self.staged_files.insert(path.to_string());
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
        let content = self.staged_files.iter().cloned().collect::<Vec<String>>().join("\n");
        fs::write(index_file, content);
    }

    pub fn get_staged_files(&self) -> Vec<String> {
        self.staged_files.iter().cloned().collect()
    }
}
   