use crate::core::index::Index;

pub fn git_add(repo_path: &str, files: Vec<&str>) {
    // 仓库根路径
    // 要添加的文件路径
    // 1. 加载现有暂存区状态
    for file_path in files {
        let mut staging_area = Index::load(repo_path);
        // 2. 将文件加入暂存区
        staging_area.stage_file(file_path);
        // 3. 持久化更新后的索引
        staging_area.persist();

        // 4. 用户反馈
        println!("Added [{}] to staging area",file_path);
    }
}