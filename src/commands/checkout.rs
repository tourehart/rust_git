use crate::core::reference::Reference;

pub fn git_checkout(
    repo_path: &str, 
    target: &str
) {
   // 1. 解析目标，检查是否为分支或提交哈希
   let target_hash = match Reference::resolve(repo_path, target) {
        Some(hash) => hash, 
        // 如果是分支，获取对应的提交哈希
        None => target.to_string(), // 如果是提交哈希，直接使用
   };
    // 2. 更新 HEAD 指向目标（分支或提交哈希）
   Reference::update_head(repo_path, &target_hash);
    // 3. 用户反馈
   println!("Switched to branch or commit [{}]",target);
}