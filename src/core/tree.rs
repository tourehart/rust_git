use std::collections::HashMap;
use super::object::Object;
use crate::utils::fs::{self, combine_paths, create_dir, read,write};


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

pub fn read_tree(repo_path: &str, tree_hash: &str) -> HashMap<String, String> {
    let object_path = format!("{}/.git/objects/{}/{}", repo_path, &tree_hash[0..2], &tree_hash[2..]);
    let raw_data = read(object_path);
    let mut content = HashMap::new();
    for line in raw_data.lines() {
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.len() == 3 {
            let mode = parts[0];
            let name = parts[1];
            let hash = parts[2];
            content.insert(name.to_string(), hash.to_string());
        }
    }
    content
}

pub fn restore_tree(repo_path: &str, base_path: &str, tree_content: &HashMap<String, String>, current_files: &mut HashMap<String, ()>) {
    for (name, hash) in tree_content {
        let relative_path = combine_paths(base_path, name);
        current_files.remove(&relative_path);
        let object_path = format!("{}/.git/objects/{}/{}", repo_path, &hash[0..2], &hash[2..]);
        let raw_data = read(object_path);
        let object = parse_object(&raw_data);
        match object {
            Object::Tree(_) => {
                // 创建目录
                create_dir(repo_path, &relative_path);
                // 递归处理子树
                let sub_tree_content = read_tree(repo_path, hash);
               restore_tree(repo_path, &relative_path, &sub_tree_content, current_files);
            }
            Object::Blob(content) => {
                // 写入文件内容
                write(combine_paths(repo_path, &relative_path), content);
            }
            _ => {}
        }
    }
}

pub fn parse_object(raw_data: &str) -> Object {
    let parts: Vec<&str> = raw_data.splitn(2, '\n').collect();
    if parts.len() == 2 {
        let header = parts[0];
        let content = parts[1];
        if header.starts_with("tree") {
            Object::Tree(content.to_string())
        } else if header.starts_with("blob") {
            Object::Blob(content.to_string())
        } else {
            Object::Commit(content.to_string())
        }
    } else {
        Object::Commit(raw_data.to_string())
    }
}

pub fn merge_trees(
    repo_path: &str,
    base_tree: &HashMap<String, String>,
    current_tree: &HashMap<String, String>,
    target_tree: &HashMap<String, String>,
) -> HashMap<String, String> {
    let mut merged_tree = HashMap::new();

    // 遍历基础树
    for (path, base_hash) in base_tree {
        let current_hash = current_tree.get(path);
        let target_hash = target_tree.get(path);

        if current_hash == target_hash {
            // 如果当前和目标分支的文件相同，则保持不变
            merged_tree.insert(path.clone(), current_hash.unwrap().clone());
        } else if current_hash == Some(base_hash) {
            // 如果当前分支的文件和基础文件相同，则采用目标分支的文件
            merged_tree.insert(path.clone(), target_hash.unwrap().clone());
        } else if target_hash == Some(base_hash) {
            // 如果目标分支的文件和基础文件相同，则采用当前分支的文件
            merged_tree.insert(path.clone(), current_hash.unwrap().clone());
        } else {
            // 冲突处理，这里简单打印冲突信息，实际中需要更复杂的处理
            println!("Conflict detected in file: {}", path);
        }
    }

    // 处理当前分支新增的文件
    for (path, current_hash) in current_tree {
        if!base_tree.contains_key(path) {
            merged_tree.insert(path.clone(), current_hash.clone());
        }
    }

    // 处理目标分支新增的文件
    for (path, target_hash) in target_tree {
        if!base_tree.contains_key(path) {
            merged_tree.insert(path.clone(), target_hash.clone());
        }
    }

    merged_tree
}