use chrono::Utc;

use super::object::Object;


// 提交对象构造器
pub struct CommitBuilder;

impl CommitBuilder {
/// 创建新提交对象
    pub fn create_commit(
        repo_path: &str,
        tree_hash: String, // 关联的树对象哈希
        parent_commit: String, // 父提交哈希
        author_info: String, // 作者信息
        commit_message: String // 提交信息
    ) -> String { // 返回新提交的哈希
        // 1. 构造提交内容：
        // let timestamp = 获取当前RFC2822格式时间;
        // let commit_content = 格式化字符串包含：
        // - tree [树哈希]
        // - parent [父提交哈希]（可选）
        // - author [作者信息] [时间戳]
        // - 空行
        // - [提交信息]
        // 2. 存储为Git对象

        let timestamp = Utc::now().to_rfc2822();
        let mut commit_content = format!("tree {}\n", tree_hash);
        commit_content.push_str(&format!("parent {}\n", parent_commit));
        commit_content.push_str(&format!("author {} {}\n", author_info, timestamp));
        commit_content.push('\n');
        commit_content.push_str(&commit_message);
        
        let commit_obj = Object::Commit(commit_content);
        commit_obj.save(repo_path) // 返回对象哈希
    }
}