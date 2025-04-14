use crate::core::repository::Repository;
use crate::core::index::Index;
use crate::core::tree::TreeProcessor;
pub fn git_commit(repo_path: &str, commit_message: &str ) {
    //提交成功的消息，包含提交的哈希值。

    // 1. 初始化仓库对象
    if !Repository::is_git_repo(repo_path){
        let repo = Repository::init(repo_path,".");
    }
        // 2. 加载当前索引
    let mut staging_index = Index::load(repo_path);
        // 3. 创建树对象哈希
    let tree_hash = TreeProcessor::create_tree(repo_path, staging_index.get_staged_files());
        // 4. 获取当前分支的最新提交
    let parent_commit = 获取当前分支提交(repo_path, "master");
        // 5. 创建新的提交对象
    let commit_hash = 创建提交(repo_path, tree_hash, parent_commit, "Author Name".to_string(), commit_message.to_string());
        // 6. 更新当前分支的引用，指向新的提交
        //更新分支引用(repo_path, "master", &commit_hash);
        // 7. 输出提交信息
    //打印"Committed changes: [commit_hash]";

}