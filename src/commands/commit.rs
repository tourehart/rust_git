use crate::core::repository::Repository;
use crate::core::index::Index;
use crate::core::tree::Tree;
use crate::core::commit::CommitBuilder;
use crate::core::reference::Reference;
use crate::utils::fs::{self, combine_paths};
pub fn git_commit(repo_path: &str, commit_message: &str ) {
    // 1. 初始化仓库对象
    if !Repository::is_git_repo(repo_path){
        let _repo = Repository::init(repo_path,".");
    }
        // 2. 加载当前索引
    let staging_index = Index::load(repo_path);
        // 3. 创建树对象哈希
    let tree_hash = Tree::create_tree(repo_path, &staging_index.get_staged_files());
        // 4. 获取当前分支的最新提交
    let parent_commit = Reference::get_current_hash(repo_path);
        // 5. 创建新的提交对象
    let commit_hash = CommitBuilder::create_commit(repo_path, tree_hash, parent_commit, "Author Name".to_string(), commit_message.to_string());
        // 6. 更新当前分支的引用，指向新的提交
    Reference::update_commit(repo_path, &commit_hash);
        // 7. 输出提交信息
    //打印"Committed changes: [commit_hash]";
    println!("Committed changes: [{}]",commit_hash);
}