use super::object::Object;

// 数据对象处理器
pub struct BlobProcessor;

impl BlobProcessor {
 /// 创建Blob对象
    pub fn create_blob(repo_path: &str, content: String) -> String { 
        // 核心处理流程：
        // 1. 将内容包装为Blob类型对象
        // 2. 调用object.save存储
        // 3. 返回生成的SHA1哈希
        let blob_obj = Object::Blob(content);
        blob_obj.save(repo_path)
    }
}