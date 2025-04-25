use crate::core::reference::Reference;

pub fn git_checkout(
    repo_path: &str, 
    target: &str
) {
   // 1. 解析目标，检查是否为分支或提交哈希
   let mut target_hash = Reference::resolve(repo_path, target);
   if target_hash == "".to_string(){
        target_hash = target.to_string();
        Reference::change_head(repo_path, &target_hash);
   }
   else {
        let ref_name = target;
        let ref_type = if ref_name.starts_with("refs/heads/") || ref_name.starts_with("refs/tags/") {
            ref_name.split('/').nth(1).unwrap()
        } else {
            "heads" // 默认使用heads
        };
        let content = format!("ref: refs/{}/{}",ref_type, ref_name.split('/').last().unwrap());
        Reference::change_head(repo_path, &content);
   }
   let root_tree_hash = Reference::get_root_hash(repo_path,&target_hash);
   Reference::restore_workspace(repo_path,&root_tree_hash);
    // 3. 用户反馈
   println!("Switched to branch or commit [{}]",target);
}