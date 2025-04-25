
use crate::core::reference::Reference;
use crate::core::commit::CommitBuilder;
use crate::core::tree::{merge_trees, read_tree, Tree};

pub fn git_merge(
    repo_path: &str, 
    branch_name: &str
) {
    
    // 1. 获取当前分支的最新提交哈希
    let current_commit_hash = Reference::get_current_hash(repo_path);
    // 2. 获取目标分支的最新提交哈希
    let target_commit_hash = Reference::resolve(repo_path, branch_name);
    
    let parent_hash = Reference::find_common_ancestor(repo_path, &current_commit_hash, &target_commit_hash);
    if let Some(base_hash) = parent_hash{
        let base_tree = read_tree(repo_path, &base_hash);
        let current_tree = read_tree(repo_path, &current_commit_hash);
        let target_tree = read_tree(repo_path, &target_commit_hash);
        // 5. 合并树对象
        let merged_tree = merge_trees(repo_path, &base_tree, &current_tree, &target_tree);
    
        // 6. 创建新的合并树对象
        let merged_tree_hash = Tree::create_tree(repo_path, &merged_tree);
    
        // 7. 创建一个新的合并提交
        let commit_hash = CommitBuilder::create_commit(
            repo_path,
            merged_tree_hash.clone(),
            format!("{}\nparent {}", current_commit_hash, target_commit_hash),
            "Author Name".to_string(),
            format!("Merge branch '{}'", branch_name).to_string()
        );

        Reference::restore_workspace(repo_path,&merged_tree_hash);
        // 8. 更新当前分支的引用，指向新的合并提交
        Reference::update_commit(repo_path, &commit_hash);
        
        // 9. 用户反馈
        println!("Merged branch '{}' into current branch. New commit hash: [{}]", branch_name, commit_hash);
    }
    else {

    }


}