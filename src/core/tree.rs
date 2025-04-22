use std::collections::HashMap;
use super::object::Object;



pub struct Tree{

    file: Vec<String>,
}

impl Tree {
    pub fn create_tree(repo_path: &str, content: &HashMap<String, String>) -> String {
        let mut sub_trees: HashMap<String, String> = HashMap::new();
        let mut files: HashMap<String, String> = HashMap::new();

        // 找出所有不同的子目录
        let mut sub_dirs = Vec::new();
        for (path, _) in content {
            if let Some(pos) = path.find('/') {
                let sub_dir = &path[0..pos];
                if !sub_dirs.contains(&sub_dir) {
                    sub_dirs.push(sub_dir);
                }
            } else {
                files.insert(path.clone(), content[path].clone());
            }
        }

        // 处理每个子目录
        for sub_dir in sub_dirs {
            let mut sub_content = HashMap::new();
            for (path, blob_hash) in content {
                if path.starts_with(sub_dir) {
                    let new_key = path.strip_prefix(&format!("{}/", sub_dir)).unwrap_or(path).to_string();
                    sub_content.insert(new_key, blob_hash.clone());
                }
            }
            let sub_tree_hash = Self::create_tree(repo_path, &sub_content);
            sub_trees.insert(sub_dir.to_string(), sub_tree_hash);
        }

        let mut tree_content:String = String::new();
        // 处理文件
        for (file_name, blob_hash) in files {
            let mode = "blob";
            let entry = format!("{} {} {}\n", mode, file_name, blob_hash);
            tree_content.push_str(&entry);
        }

        // 处理子目录
        for (sub_dir, sub_tree_hash) in sub_trees {
            let mode = "tree";
            let entry = format!("{} {} {}\n", mode, sub_dir, sub_tree_hash);
            tree_content.push_str(&entry);
        }

        let tree_obj = Object::Tree(tree_content);
        tree_obj.save(repo_path)

    }

}