use crate::core::index::Index;
use crate::core::blob::BlobProcessor;
use crate::utils::fs::{self, read};
pub fn git_add(repo_path: &str, files: Vec<&str>) {
    // 仓库根路径
    // 要添加的文件路径
    // 1. 加载现有暂存区状态
    for file_path in files {
        let mut staging_area = Index::load(repo_path);
        let content = read(file_path.to_owned());
        let hash = BlobProcessor::create_blob(repo_path, content);
        staging_area.stage_file(file_path,hash);
        println!("Added [{}] to staging area",file_path);
    }
}