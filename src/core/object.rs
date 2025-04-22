use crate::utils::fs::create_dir;
use crate::utils::fs::write;
use crate::utils::hash;
use crate::utils::fs;
// 核心对象类型枚举
pub enum Object {
    Commit(String), // 提交对象：存储提交信息
    Tree(String),  // 树对象：存储目录结构
    Blob(String), 
}

impl Object {
    /// 对象存储算法
    // 数据对象：存储文件内容
   pub fn save(&self, repo_path: &str) -> String {
    // 1. 序列化对象数据
        let raw_data = match self {
            Object::Commit(data) => format!("commit {}\n{}", data.len(), data),
            Object::Tree(data) => format!("tree {}\n{}", data.len(), data),
            Object::Blob(data) => format!("blob {}\n{}",data.len(), data),
        };

        // 2. 计算SHA1哈希值
        let hash = hash::sha1(&raw_data);

        // 3. 存储到objects目录
        let dir = format!("{}/.git/objects", repo_path);
        create_dir(&dir,&hash[0..2]);
        write(format!("{}/{}/{}", dir, &hash[0..2], &hash[2..]),raw_data);
        hash // 返回对象哈希
   }
   
   
}