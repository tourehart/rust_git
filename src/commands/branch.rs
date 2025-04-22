use crate::core::reference::Reference;

pub fn git_branch(
    repo_path: &str, 
   branch_name: &str, 
   delete: bool
) {

    if delete {
        // 1. 删除分支
        Reference::delete(repo_path, branch_name);
        // 2. 用户反馈
        println!("Deleted branch [{}]",branch_name);
    } else {
            // 1. 获取当前分支的最新提交哈希
        let ref_name = Reference::get_current_ref(repo_path);
        let commit_hash = Reference::resolve(repo_path, &ref_name);
            // 2. 创建新分支
        Reference::create(repo_path, branch_name, &commit_hash.unwrap());
            // 3. 用户反馈打印
        println!("Created branch [{}]",branch_name);
    }

}