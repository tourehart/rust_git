use sha1::{Digest, Sha1};

pub fn sha1(key: &str) -> String {
    let mut hasher = Sha1::new();
    hasher.update(key);
    let result = hasher.finalize();
    format!("{:x}", result)
}