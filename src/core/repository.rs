use std::path::Path;

use crate::utils::fs::{self, combine_paths, create_dir, create_file,write};

// 仓库结构体，用于存储仓库路径
pub struct Repository {
    path: String, // 仓库路径
}

impl Repository {
    // 初始化新仓库
    // 创建.git目录及其子目录结构，包括.refs和.objects目录，同时创建.HEAD文件
    // 参数path为仓库的初始化路径，默认为当前目录
    pub fn init(cli_path: &str,path: &str) -> Self {
        let mut repo_path = combine_paths(cli_path, path);
        // 创建.git目录
        create_dir(&repo_path,".git");
        // 创建.git/refs目录
        create_dir(&repo_path,".git/refs");
        create_dir(&repo_path,".git/refs/heads");
        create_dir(&repo_path,".git/refs/tags");
        // 创建.git/objects目录
        create_dir(&repo_path,".git/objects");
        // 创建.git/HEAD文件
        create_file(&repo_path,".git/HEAD");
        write(combine_paths(&repo_path, ".git/HEAD"),"ref: refs/heads/main".to_string());
        Repository {
            path: repo_path,
        }
    }

    // 验证当前目录是否为Git仓库
    // 通过检查当前目录下是否存在.git目录来判断
    pub fn is_git_repo(path: &str) -> bool {
        let git_dir = Path::new(path).join(".git");
        git_dir.is_dir()
    }
}
