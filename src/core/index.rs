// 索引结构体定义
pub struct Index {
    repo_path: String, 
   staged_files: HashSet<String> // 暂存文件集合
}

impl Index {
    /// 加载现有索引
    pub fn load(repo_path: &str) -> Self {
        let index_file = format!("{}/.git/index", repo_path);
        // 读取逻辑伪代码：
        if 索引文件存在 {
            将文件内容解析为文件路径集合
        } else {
            初始化空集合
        }

        Index { 
            //返回初始化后的实例
        }
    }

    /// 添加文件到暂存区
    pub fn stage_file(&mut self, path: &str) {
        // 核心操作：
    }
    
    /// 从暂存区移除文件
    pub fn unstage_file(&mut self, path: &str) {
        // 核心操作：
    }

    /// 内部保存方法
    fn persist(&self) {
        // 存储格式：
        // 关联的仓库路径
    }
}
   