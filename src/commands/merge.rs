
use crate::core::reference::Reference;
use crate::core::commit::CommitBuilder;
pub fn git_merge(
    repo_path: &str, 
    branch_name: &str
) {
    
    // 1. 获取当前分支的最新提交哈希
    let current_branch = "master"; // 假设当前分支为 master
    let current_commit_hash = Reference::resolve(repo_path, current_branch);
    // 2. 获取目标分支的最新提交哈希
    let target_commit_hash = Reference::resolve(repo_path, branch_name);
    // 3. 创建一个新的合并提交
    let merge_commit_hash = CommitBuilder::create_commit(
        repo_path, 
        "tree_hash_placeholder".to_owned(), 
        current_commit_hash, 
        "Author Name".to_string(), 
        "Merge branch [branch_name]".to_owned()
    );

    // 4. 更新当前分支的引用，指向新的合并提交
    Reference::create(repo_path, current_branch, &merge_commit_hash);

    // 5. 用户反馈
    println!("Merge branch [branch_name] into [{}]",current_branch);
}