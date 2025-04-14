use crate::core::index::Index;


// 文件移除命令伪实现
pub fn git_remove(repo_path: &str, files: Vec<&str>, force_mode: bool) {
    // 仓库根路径
   // 要移除的文件路径
   // 是否强制删除工作区文件
   // 1. 加载当前暂存区状态
   for file_path in files {
        let mut staging_area = Index::load(repo_path);

        staging_area.unstage_file(file_path);

        if force_mode {
            //删除工作区文件(file_path);
        }

        staging_area.persist();


        println!("Added [{}] to staging area",file_path);
    }
}