use std::path::{Path, PathBuf};

pub fn tmp_path() -> PathBuf {
    PathBuf::from("tests")
        .join("tmp")
        .canonicalize()
        .expect("canon tmp path")
}

pub fn test_path(subfolder: &str) -> PathBuf {
    PathBuf::from("tests")
        .join("data")
        .join("acceptance")
        .join(subfolder)
        .canonicalize()
        .expect("canon test path")
}

pub fn tear_down<P: AsRef<Path>>(target_path: P) {
    std::fs::remove_dir_all(target_path).expect("tear_down - rm -rf");
}
