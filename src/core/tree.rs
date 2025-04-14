use crate::utils::hash::sha1;

use super::object::Object;



pub struct TreeProcessor;

impl TreeProcessor {
 /// 创建Blob对象
    pub fn create_tree(repo_path: &str, content: Vec<String>) -> String { 
        // 核心处理流程：
        // 仓库路径
        // 文件原始内容
        // 返回对象哈希
        // 1. 将内容包装为Blob类型对象
        // 2. 调用object.save存储
        // 3. 返回生成的SHA1哈希
        let tree_content = "".to_string();
        
        let tree_obj = Object::Tree(tree_content);
        sha1("")
    }
}