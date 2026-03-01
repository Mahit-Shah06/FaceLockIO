use sha2::{Sha256, Digest};

pub fn hash_id(name: &str) -> String {
    let mut hasher = Sha256::new();

    hasher.update(name.trim().to_lowercase().as_bytes());
    let result = hasher.finalize();

    format!("{:x}", result)
}
