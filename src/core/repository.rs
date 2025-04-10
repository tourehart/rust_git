

// Git仓库核心模块
mod repository {
    // 引入文件和目录操作相关的标准库
    use std::fs;
    use std::path::Path;

    // 仓库结构体，用于存储仓库路径
    pub struct Repository {
        path: String, // 仓库路径
    }

    impl Repository {
        // 初始化新仓库
        // 创建.git目录及其子目录结构，包括.refs和.objects目录，同时创建.HEAD文件
        // 参数path为仓库的初始化路径，默认为当前目录
        pub fn init(path: &str) -> Self {
            // 创建.git目录
            let git_dir = Path::new(path).join(".git");
            fs::create_dir(&git_dir).expect("Failed to create.git directory");

            // 创建.git/refs目录
            let refs_dir = git_dir.join("refs");
            fs::create_dir(&refs_dir).expect("Failed to create.git/refs directory");

            // 创建.git/objects目录
            let objects_dir = git_dir.join("objects");
            fs::create_dir(&objects_dir).expect("Failed to create.git/objects directory");

            // 创建.git/HEAD文件
            let head_file = git_dir.join("HEAD");
            fs::File::create(&head_file).expect("Failed to create.git/HEAD file");

            // 返回初始化后的仓库实例，包含仓库路径
            Repository {
                path: path.to_string(),
            }
        }

        // 验证当前目录是否为Git仓库
        // 通过检查当前目录下是否存在.git目录来判断
        pub fn is_git_repo(path: &str) -> bool {
            let git_dir = Path::new(path).join(".git");
            git_dir.is_dir()
        }
    }
}