use crate::core::repository::Repository;

pub fn git_init(repo_path: &str,path: &str){

    Repository::init(repo_path,path);
    println!("已在[{}]初始化空的Git仓库", path);
    
}